use rand::Rng;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
// struct Metric {
//     worker_id: usize,
//     latency_ms: u64,
//     success: bool,
// }

// async fn worker(
//     id: usize,
//     tx: mpsc::Sender<Metric>,
// ) {
//     loop {
//         let latency = rand::thread_rng()
//             .gen_range(50..300);

//         let success = rand::thread_rng()
//             .gen_bool(0.9);

//         let metric = Metric {
//             worker_id: id,
//             latency_ms: latency,
//             success,
//         };

//         tx.send(metric)
//             .await
//             .unwrap();

//         sleep(Duration::from_millis(500)).await;
//     }
// }

#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel(100);

    println!("{:?}", tx.is_closed());
    // for id in 0..4 {
    //     let tx_clone = tx.clone();

    //     tokio::spawn(async move {
    //         worker(id, tx_clone).await;
    //     });
    // }

    // drop(tx);

    // let mut total_requests = 0;
    // let mut total_errors = 0;
    // let mut total_latency = 0u64;

    // loop {

    //     tokio::select! {

    //         Some(metric) = rx.recv() => {

    //             total_requests += 1;

    //             if !metric.success {
    //                 total_errors += 1;
    //             }

    //             total_latency += metric.latency_ms;

    //             println!(
    //                 "Received metric: {:?}",
    //                 metric
    //             );
    //         }

    //         _ = sleep(Duration::from_secs(5)) => {

    //             if total_requests > 0 {

    //                 let avg_latency =
    //                     total_latency as f64
    //                     / total_requests as f64;

    //                 println!();
    //                 println!("===== METRICS REPORT =====");
    //                 println!("Requests: {}", total_requests);
    //                 println!("Errors: {}", total_errors);
    //                 println!("Average latency: {:.2} ms", avg_latency);
    //                 println!("==========================");
    //                 println!();
    //             }
    //         }
    //     }
    // }
}