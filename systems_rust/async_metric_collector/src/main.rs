use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct Metric {
    source: String,
    value: i32,
}

async fn producer(
    id: i32,
    tx: mpsc::Sender<Metric>,
) {
    for i in 1..=5 {
        let metric = Metric {
            source: format!("server-{}", id),
            value: i * 10,
        };

        println!("Produced: {:?}", metric);

        tx.send(metric).await.unwrap();

        sleep(Duration::from_secs(1)).await;
    }

    println!("Producer {} finished", id);
}

async fn collector(
    mut rx: mpsc::Receiver<Metric>,
) {
    while let Some(metric) = rx.recv().await {
        println!("Collected: {:?}", metric);
    }

    println!("Collector exiting");
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    // Spawn producers
    for id in 1..=3 {
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            producer(id, tx_clone).await;
        });
    }

    drop(tx);

    collector(rx).await;
}