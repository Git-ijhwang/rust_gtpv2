mod gtpv2_type;
mod gtpv2_send;
mod gtpv2_recv;
mod gtpv2_ie;
mod gtp_msg;
mod config;
mod udpsock;
mod pkt_manage;
mod gtp_dictionary;
mod validate_gtpv2;
mod peers;
mod ippool;
mod session;
mod dump;
mod msg_static;
// mod api_client;

use std::fs;
use std::io::Error;
use std::net::AddrParseError;
use core::result::Result;
use thiserror::Error;
use chrono::Local;
use std::path::Path;
use std::{fs::File, fs::metadata,io::Write};
use log::{debug, error, info, trace, warn};
use reqwest::Client;
// use std::sync::Mutex;
use tokio::sync::Mutex;
// use std::sync::Arc;



use crate::peers::*;
use crate::config::*;
use crate::udpsock::*;
use crate::gtpv2_recv::*;
use crate::gtp_dictionary::*;
use crate::ippool::*;
use crate::pkt_manage::*;
use crate::msg_static::*;
// use crate::api_client::run_mmc_communication;


#[derive(Debug, Error)]
pub enum MyError {
    #[error("Address parsing error: {0}")]
    AddrError(AddrParseError),

    #[error("Unknown error occurred")]
    Unknown,
}


fn check_and_roll_log(log_filename: &str) -> std::io::Result<File> {
    let max_size = 10 * 1024 * 1024; // 10MB
    let log_path = Path::new(log_filename);

    if let Ok(file_metadata) = metadata(log_path) {
        if file_metadata.len() > max_size {
            let new_filename = format!("{}-{}", log_filename, Local::now().format("%Y-%m-%d-%H-%M-%S"));
            println!("Log file size exceeded 10MB, rolling to: {}", new_filename);
            return Ok(File::create(new_filename)?);
        }
    }
    
    File::create(log_filename)
}


async fn setup_logger() -> Result<(), fern::InitError> {

    let log_dir = "logs";
    if !fs::metadata(log_dir).is_ok() {
        fs::create_dir(log_dir).unwrap(); // create directory for log file
    }

    let log_filename = format!("{}/logging_{}.log", log_dir, Local::now().format("%Y-%m-%d"));

    // 파일 롤링 처리: 로그 파일 크기를 확인하여 10MB 초과 시 새로운 파일 생성
    let log_file = check_and_roll_log(&log_filename).unwrap();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                // humantime::format_rfc3339_seconds(SystemTime::now()),
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        // .chain(std::io::stdout())
        // .chain(fern::log_file(log_filename)?)
        .chain(log_file)
        .apply()?;

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error>
{
    /* Read Config */
    if let Err(v) = read_conf("src/config/config", false).await {
        println!("Failed Open file {}", v);
        return Err(v);
    }

    if let Err(v) = read_conf("src/config/config_peer", true).await {
        println!("Failed Open file {}", v);
        return Err(v);
    }

    //Set up logger
    if let Err(ret) = setup_logger().await {
        println!("Failed to set up logger: {:?}", ret);
        return Err(Error::new(std::io::ErrorKind::Other, "Failed to set up logger"));
    }

    if let Err(ret) = load_gtp_dictionary("src/config/GTPv2_Dictionary.json").await {
        println!("Failed to load GTPv2 Dictionary: {:?}", ret);
        return Err(Error::new(std::io::ErrorKind::Other, "Failed to load GTPv2 Dictionary"));
    }

    //Create Peer structure based on config_peer file
    create_peer().await;

    //IP pool
    prepare_ip_pool().await;

    info!("Peer Manage");
    let peer_handle = tokio::spawn(async move { peer_manage().await; });

    info!("Queue Manage");
    let queue_handle = tokio::spawn(async move { check_timer().await; });

	info!("MMC Task Start");
    let client = Client::new();
	let api_handle = tokio::spawn( {
        let client = client.clone(); // 클론하여 소유권 넘기기

		async move {
			let config = CONFIG_MAP.read().await;

			if let Some(addr) = config.get("API_SERVER").clone() {
				// run_mmc_communication(addr.as_str()).await;
                //url format: "127.0.0.1::8080"
				if let Err(e) = report_stats(&client.clone(), addr.as_str()).await {
					// report_stats(&client, addr.as_str()).await;
					eprintln!("Error reporting stats: {}", e);
				}
			}
			else {
				eprintln!("Failed to fall to call function for api server");
			}
			()
		}
	});

	info!("Receive Task Start");
	let config = CONFIG_MAP.read().await;
	if let Some(src_port) = config.get("SrcPort").clone() {
		if let Ok(recv_socket) = socket_create( format!("0.0.0.0:{}", src_port)) {
			info!("Socket Successfully Created!: {:?}", recv_socket);
			recv_task(recv_socket).await;
		}
		else {
			eprintln!("Failed to create socket for address 0.0.0.0:{}", src_port);
		}
	}

	let _ = tokio::join!(peer_handle, queue_handle, api_handle);

    Ok(())
}
