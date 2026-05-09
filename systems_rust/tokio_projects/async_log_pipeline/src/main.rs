use rand::Rng;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
struct LogEvent {
    producer_id: usize,
    message: String,
}

async fn producer(
    id: usize,
    tx: mpsc::Sender<LogEvent>,
    token: CancellationToken,
) {

    let mut counter = 0;

    loop {

        tokio::select! {

            _ = token.cancelled() => {

                println!("Producer {} shutting down", id);

                break;
            }

            _ = sleep(Duration::from_millis(
                rand::thread_rng().gen_range(200..600)
            )) => {

                let log = LogEvent {

                    producer_id: id,

                    message: format!(
                        "log message {}",
                        counter
                    ),
                };

                counter += 1;

                if tx.send(log).await.is_err() {

                    break;
                }
            }
        }
    }
}

async fn worker(
    id: usize,
    mut rx: mpsc::Receiver<LogEvent>,
    batch_tx: mpsc::Sender<Vec<LogEvent>>,
    token: CancellationToken,
) {

    let mut batch = Vec::new();

    loop {

        tokio::select! {

            _ = token.cancelled() => {

                println!("Worker {} shutting down", id);

                break;
            }

            msg = rx.recv() => {

                match msg {

                    Some(log) => {

                        println!(
                            "Worker {} processing {:?}",
                            id,
                            log
                        );

                        batch.push(log);

                        if batch.len() >= 5 {

                            let to_send =
                                std::mem::take(&mut batch);

                            if batch_tx.send(to_send)
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }

                    None => break,
                }
            }
        }
    }

    if !batch.is_empty() {

        let _ = batch_tx.send(batch).await;
    }
}

async fn writer(
    mut batch_rx: mpsc::Receiver<Vec<LogEvent>>,
    token: CancellationToken,
) {

    loop {

        tokio::select! {

            _ = token.cancelled() => {

                println!("Writer shutting down");

                break;
            }

            batch = batch_rx.recv() => {

                match batch {

                    Some(logs) => {

                        println!();
                        println!(
                            "WRITING BATCH: {} logs",
                            logs.len()
                        );

                        for log in logs {

                            println!(
                                "[Producer {}] {}",
                                log.producer_id,
                                log.message
                            );
                        }

                        println!();
                    }

                    None => break,
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let token = CancellationToken::new();

    let (log_tx, log_rx) =
        mpsc::channel::<LogEvent>(100);

    let (batch_tx, batch_rx) =
        mpsc::channel::<Vec<LogEvent>>(20);

    for producer_id in 0..3 {

        let tx_clone = log_tx.clone();

        let token_clone = token.clone();

        tokio::spawn(async move {

            producer(
                producer_id,
                tx_clone,
                token_clone
            ).await;
        });
    }

    let worker_token = token.clone();

    let batch_tx_clone = batch_tx.clone();

    tokio::spawn(async move {

        worker(
            1,
            log_rx,
            batch_tx_clone,
            worker_token
        ).await;
    });

    let writer_token = token.clone();

    tokio::spawn(async move {

        writer(batch_rx, writer_token).await;
    });

    sleep(Duration::from_secs(10)).await;

    println!();
    println!("INITIATING SHUTDOWN");
    println!();

    token.cancel();

    sleep(Duration::from_secs(3)).await;

    println!("System shutdown complete");
}