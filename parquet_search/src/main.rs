
use chrono::DateTime;
use chrono::{TimeZone, Utc};

// ./_meta/logs/query_recommendations/0/2026/06/09/08/84ba92cc0ff10292/xyz.parquet
fn main() {
    println!("Hello, world!");

    let parquet_paths: Vec<&str> = vec![
        "./_meta/logs/query_recommendations/0/2026/06/09/08/84ba92cc0ff10292/xyz.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/09/09/84ba92cc0ff10292/abc.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/09/10/84ba92cc0ff10292/pqr.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/09/11/84ba92cc0ff10292/lmn.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/09/12/84ba92cc0ff10292/uvw.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/10/08/84ba92cc0ff10292/def.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/10/09/84ba92cc0ff10292/ghi.parquet",
        "./_meta/logs/query_recommendations/0/2026/06/10/10/84ba92cc0ff10292/jkl.parquet",
    ];

    // println!("{:?}", parquet_paths);
    let hour_timestamps_ns = paths_to_hour_nanoseconds(&parquet_paths);
     
    // 1780995600000000000
    // 1781082000000000000

    let start_time_ns: i64 = 1780995600000000000_i64; // 2026-06-09 08:00:00 UTC
    let end_time_ns: i64 = 1781082000000000000_i64;   // 2026-06-10 08:00:00 UTC

    
    let l = 0;
    let r = hour_timestamps_ns.len()-1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if hour_timestamps_ns[mid] < start_time_ns {
                l = mid + 1;
            } else if hour_timestamps_ns[mid] > end_time_ns {
                r = mid - 1;
            } else {
                println!("Found timestamp in range: {}", hour_timestamps_ns[mid]);
                break;
            }
        }
    
    

    // println!("{:?}", hour_timestamps_ns);

    // let times = time_range_strings("09:00", "12:00");
    // println!("{:?}", times);









}

fn paths_to_hour_nanoseconds(paths: &[&str]) -> Vec<i64> {
    paths
        .iter()
        .filter_map(|path| {
            let clean = path.trim_start_matches("./");
            let parts: Vec<&str> = clean.split('/').collect();

            let base_idx = parts.iter().position(|p| *p == "query_recommendations")?;

            let year: i32 = parts.get(base_idx + 2)?.parse().ok()?;
            let month: u32 = parts.get(base_idx + 3)?.parse().ok()?;
            let day: u32 = parts.get(base_idx + 4)?.parse().ok()?;
            let hour: u32 = parts.get(base_idx + 5)?.parse().ok()?;

            if hour > 23 {
                return None;
            }

            let dt: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).single()?;
            Some(dt.timestamp().saturating_mul(1_000_000_000))
        })
        .collect()
}

// fn time_range_strings(start_time: &str, end_time: &str) -> Vec<String> {
//     let start_hour: i32 = start_time
//         .split(':')
//         .next()
//         .unwrap_or("0")
//         .parse()
//         .unwrap_or(0);
//     let end_hour: i32 = end_time
//         .split(':')
//         .next()
//         .unwrap_or("0")
//         .parse()
//         .unwrap_or(0);

//     let mut result = Vec::new();

//     if start_hour <= end_hour {
//         for hour in start_hour..=end_hour {
//             result.push(format!("{:02}:00", hour));
//         }
//     }

//     result
// }
