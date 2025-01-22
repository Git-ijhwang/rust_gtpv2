use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use tokio::time;
use std::collections::HashMap;

use crate::msg_static::*;

#[derive(Serialize)]
struct PgwStatus {
    cpu_usage: u8,
    mem_usage: u32,
}

async fn send_status_to_server(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
    let status = PgwStatus {
        cpu_usage: 75,
        mem_usage: 2023,
    };

    let response = client
        .post(url)
        .json(&status)
        .send()
        .await?;

    if response.status().is_success() {
        println!("successfully sent status to server!");
    }
    else {
        eprintln!("Failed to send status: {}", response.status());
    }

    Ok(())
}


pub async fn run_mmc_communication(addr: &str) {//} -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url_string = format!("https://{}", addr);
    let url: &str = url_string.as_str();
    let stats: SharedStats = Arc::new(Mutex::new(HashMap::new()));

    loop {
        if let Err(e) = send_status_to_server(&client, url.clone()).await {
            eprintln!("Error sending status: {}", e);
        }

        if let Err(e) = report_stats_with_time(&client, url.clone(), &stats).await {
            eprintln!("Error reporting stats with timestamps: {}", e);
        }

        time::sleep(Duration::from_secs(5)).await;
    }
}