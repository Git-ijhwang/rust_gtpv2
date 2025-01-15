use tokio::time;
use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MessageStats {
    sent: u64,
    received: u64,
    timestamps: Vec<DateTime<Utc>>,
}

// 메시지 타입별 통계를 저장하는 타입
pub type StatsMap = HashMap<u8, MessageStats>;

// 공유 가능한 상태를 위한 타입 정의
pub type SharedStats = Arc<Mutex<StatsMap>>;



// 메시지 송수신 통계와 시간을 업데이트하는 함수
fn update_stats_with_time(stats: &SharedStats, msg_type: u8, is_sent: bool) {
    let mut stats_map = stats.lock().unwrap();
    let entry = stats_map.entry(msg_type).or_default();

    // 송수신 카운트 업데이트
    if is_sent {
        entry.sent += 1;
    } else {
        entry.received += 1;
    }

    // 현재 시간 기록
    entry.timestamps.push(Utc::now());
}


#[derive(Serialize)]
struct ReportData {
    message_type: u8,
    sent: u64,
    received: u64,
    timestamps: Vec<String>, // ISO 8601 형식으로 변환된 시간
}

fn generate_and_reset_report_data(stats: &SharedStats) -> Vec<ReportData> {
    let mut stats_map = stats.lock().unwrap();
    let report_data: Vec<ReportData> = stats_map
        .iter()
        .map(|(&msg_type, stat)| ReportData {
            message_type: msg_type,
            sent: stat.sent,
            received: stat.received,
            timestamps: stat.timestamps.iter().map(|ts| ts.to_rfc3339()).collect(),
        })
        .collect();

    // 데이터 리셋
    stats_map.clear();
    report_data
}

// REST API로 통계 보고
pub async fn report_stats_with_time(client: &Client, url: &str,
    stats: &SharedStats)
-> Result<(), Box<dyn Error>> {
    let report_data = generate_and_reset_report_data(stats);

    let response = client
        .post(url)
        .json(&report_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully reported stats with timestamps to server!");
    } else {
        eprintln!("Failed to report stats: {}", response.status());
    }

    Ok(())
}

// 메시지 처리 함수 예제
fn process_message_with_time(stats: &SharedStats, msg_type: u8) {
    // 메시지 수신 처리
    println!("Processing received message of type: {}", msg_type);
    update_stats_with_time(stats, msg_type, false);

    // 메시지 송신 처리 (예제)
    println!("Sending response for message type: {}", msg_type);
    update_stats_with_time(stats, msg_type, true);
}


// #[tokio::main]
// async fn main() {
//     let stats: SharedStats = Arc::new(Mutex::new(HashMap::new()));
//     let client = Client::new();
//     let url = "http://server-b.local/api/pgw/stats_with_time";

//     tokio::spawn(async move {
//         // 메시지 처리 시뮬레이션
//         loop {
//             process_message_with_time(&stats, 1); // 메시지 타입 1
//             process_message_with_time(&stats, 2); // 메시지 타입 2
//             time::sleep(time::Duration::from_secs(1)).await;
//         }
//     });

//     // 5초마다 통계 보고
//     loop {
//         time::sleep(time::Duration::from_secs(5)).await;
//     }
// }
