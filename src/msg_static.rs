use reqwest::Client;
// use tokio::sync::Mutex;
// use std::sync::Mutex;
use serde::Serialize;
use chrono::{DateTime, Utc};
use sysinfo::{System};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};


#[derive(Serialize, Clone)]
pub struct CpuStats {
    pub usage: f32,
    pub timestamps: DateTime<Utc>,
}

#[derive(Serialize, Clone)]
pub struct MemStats {
    pub usage: f32,
    timestamps: DateTime<Utc>,
}

#[derive(Serialize, Clone)]
pub struct MessageStats {
    message_type:   u8,
    success:        bool,
    error:          u8,
    timestamps:     DateTime<Utc>,
}

impl MessageStats {
    pub fn new() -> Self {
        MessageStats {
            message_type: 0,
            success: false,
            error: 0,
            timestamps: Utc::now(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ReportData {
    pub cpu_data: Vec<CpuStats>,
    pub mem_data: Vec<MemStats>,
    pub msg_data: Vec<MessageStats>,
}


pub type MessageStatsData = Vec<MessageStats>;

// lazy_static::lazy_static! {
//     pub static ref MSG_STAT: Arc<Mutex<MessageStatsData>> = Arc::new(Mutex::new(MessageStatsData::new()));
// }

lazy_static::lazy_static! {
    pub static ref MSG_STAT: Mutex<Vec<MessageStats>> = Mutex::new(Vec::new());

}

pub async fn update_message_stats (message_type: u8, success: bool, error: u8) {
    let mut msg = MSG_STAT.lock().await;

    msg.push(MessageStats {
        message_type,
        success,
        error,
        timestamps: Utc::now(),
    });
}

fn update_cpu_stats(cpu_data: &mut Vec<CpuStats>, usage: f32) {
    cpu_data.push(CpuStats {
        usage,
        timestamps: Utc::now(),
    });
}

fn update_mem_stats(mem_data: &mut Vec<MemStats>, usage: f32) {
    mem_data.push(MemStats {
        usage,
        timestamps: Utc::now(),
    });
}

impl CpuStats {
    pub fn new() -> Self {
        CpuStats {
            usage: 0.0,
            timestamps: Utc::now(),
        }
    }
}

pub async fn report_stats(client: &Client, url: &str)
-> Result<(), Box<dyn std::error::Error>>
{
    let mut system = System::new_all();

    let mut cpu_data: Vec<CpuStats> = Vec::new();
    let mut mem_data: Vec<MemStats> = Vec::new();
    let mut cnt = 0;

    loop {
        cnt += 1;

        // let msg = MSG_STAT.lock().unwrap();
        let msg = MSG_STAT.lock().await;
        system.refresh_cpu_all();
        system.refresh_memory();

        let total_cpu_usage = system.global_cpu_usage();//.cpu_usage();
        update_cpu_stats(&mut cpu_data, total_cpu_usage);

        let total_memory = system.total_memory() as f32;
        let used_memory = system.used_memory() as f32;
        let memory_usage = (used_memory / total_memory) * 100.0; // 퍼센트로 변환
        update_mem_stats(&mut mem_data, memory_usage);

        // std::thread::sleep(std::time::Duration::from_secs(1)).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        if cnt > 5 {
            cnt = 0;

            let report_data = ReportData {
                cpu_data: cpu_data.clone(),
                mem_data: mem_data.clone(),
                msg_data: msg.clone(),
            };

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

            cpu_data.clear();
            mem_data.clear();
        }
    }

    Ok(())
}


/*
// 메시지 타입별 통계를 저장하는 타입
pub type StatsMap = HashMap<u8, MessageStats>;

// 공유 가능한 상태를 위한 타입 정의
pub type SharedStats = Arc<Mutex<StatsMap>>;

// 메시지 송수신 통계와 시간을 업데이트하는 함수
pub async fn update_stats_with_time(
    // stats: &SharedStats,
    msg_type: u8, is_sent: bool) {
    // let mut stats_map = stats.lock().unwrap();
    let mut stats_map = SharedStats.lock().unwrap();
    let entry = stats_map.entry(msg_type).or_default();

    if is_sent {
        entry.sent += 1;
    } else {
        entry.received += 1;
    }

    entry.timestamps.push(Utc::now());
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

*/