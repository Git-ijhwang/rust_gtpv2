mod gtpv2_type;
mod gtpv2_send;
mod gtpv2_recv;
mod gtpv2_ie;
mod gtp_msg;
mod config;
mod udpsock;
mod packet;
mod timer;
mod gtp_dictionary;
mod validate_gtpv2;
mod peers;
mod ippool;
mod session;

use std::sync::{Arc, RwLock, Mutex};
use config::*;
use gtp_msg::*;
use std::thread;
use std::io::Error;
use core::result::Result;
use tokio::time::{self, Duration};
use crate::peers::*;
use crate::udpsock::*;
use crate::gtpv2_recv::*;
use crate::gtp_dictionary::*;
use crate::session::*;
use crate::ippool::*;
use crate::timer::*;
use std::net::AddrParseError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MyError {
    #[error("Address parsing error: {0}")]
    AddrError(AddrParseError),

    #[error("Unknown error occurred")]
    Unknown,
}


#[tokio::main]
async fn main() -> Result<(), Error>
{
    /* Read Config */
    if let Err(v) = read_conf("src/config/config", false) {
        println!("Failed Open file {}", v);
        return Err(v);
    }

    if let Err(v) = read_conf("src/config/config_peer", true) {
        println!("Failed Open file {}", v);
        return Err(v);
    }

    if let Err(v) = load_gtp_dictionary("src/config/GTPv2_Dictionary.json"){
        println!("Failed load dictionary file");
        return Err(v);
    }

    //Create Peer structure based on config_peer file
    create_peer();

    //IP pool
    prepare_ip_pool();

    let session_list = Arc::new(Mutex::new(SessionList::new()));
    let teid_list = Arc::new(Mutex::new(TeidList::new()));
    let msg_queue = MsgQue::new();

    let config = CONFIG_MAP.read().unwrap();

    if let Ok(recv_socket) = socket_create( format!("0.0.0.0:{}", config.get("SrcPort").unwrap().to_string())) {
        println!("Socket Successfully Created!: {:?}", recv_socket);
        thread::spawn(move || gtpv2_recv_task(recv_socket, session_list, teid_list));
    }
    else{
        eprintln!("Failed to create socket for address 0.0.0.0:{}",
        config.get("SrcPort").unwrap());
    }

    peer_manage().await;

    let queue_clone = SHARED_QUEUE.clone();
    queue_clone.lock().await.check_timer().await;

    Ok(())
}
