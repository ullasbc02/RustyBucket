// using Tokio to perform concurrent downloads of files, simulating the download process with sleep
// -------------------------------------------------------------------------------------------
use tokio::time::{sleep, Duration};

async fn download_file(file_name: &str, seconds: u64) {
    println!("Starting download: {}", file_name);

    sleep(Duration::from_secs(seconds)).await;

    println!("Finished download: {}", file_name);
}

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        download_file("file1.zip", 3).await;
    });

    let task2 = tokio::spawn(async {
        download_file("file2.zip", 1).await;
    });

    let task3 = tokio::spawn(async {
        download_file("file3.zip", 2).await;
    });

    task1.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();

    println!("All downloads complete");
}

// using Tokio to perform concurrent downloads of files, making actual HTTP requests
// -------------------------------------------------------------------------------------------
// use reqwest;
// use tokio::time::{Instant};

// async fn download_url(url: &str) {
//     println!("Starting: {}", url);

//     let start = Instant::now();

//     let response = reqwest::get(url)
//         .await
//         .unwrap();

//     let body = response.text()
//         .await
//         .unwrap();

//     let elapsed = start.elapsed();

//     println!(
//         "Finished: {} | bytes={} | time={:?}",
//         url,
//         body.len(),
//         elapsed
//     );
// }

// #[tokio::main]
// async fn main() {
//     let task1 = tokio::spawn(async {
//         download_url("https://example.com").await;
//     });

//     let task2 = tokio::spawn(async {
//         download_url("https://httpbin.org/delay/2").await;
//     });

//     let task3 = tokio::spawn(async {
//         download_url("https://www.rust-lang.org").await;
//     });

//     task1.await.unwrap();
//     task2.await.unwrap();
//     task3.await.unwrap();

//     println!("All downloads complete");
// }

// Without tokio
// -------------------------------------------------------------------------------------------
// use reqwest::blocking; // Use the blocking module
// use std::time::Instant;

// fn download_url(url: &str) {
//     println!("Starting: {}", url);

//     let start = Instant::now();

//     // No .await here. The program "freezes" on this line until the response arrives.
//     let response = blocking::get(url).unwrap();

//     // Again, no .await. It waits until the entire body is downloaded.
//     let body = response.text().unwrap();

//     let elapsed = start.elapsed();

//     println!(
//         "Finished: {} | bytes={} | time={:?}",
//         url,
//         body.len(),
//         elapsed
//     );
// }

// fn main() {
//     // These will now run one after the other in strict order.
//     download_url("https://example.com");
//     download_url("https://httpbin.org/delay/2");
//     download_url("https://www.rust-lang.org");

//     println!("All downloads complete");
// }

