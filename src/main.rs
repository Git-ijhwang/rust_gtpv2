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

use std::thread;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::net::AddrParseError;
use config::*;
use core::result::Result;
use thiserror::Error;
use tokio::sync::RwLock;
use crate::peers::*;
use crate::udpsock::*;
use crate::gtpv2_recv::*;
use crate::gtp_dictionary::*;
use crate::session::*;
use crate::ippool::*;
use crate::pkt_manage::*;


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
    if let Err(v) = read_conf("src/config/config", false).await {
        println!("Failed Open file {}", v);
        return Err(v);
    }

    if let Err(v) = read_conf("src/config/config_peer", true).await {
        println!("Failed Open file {}", v);
        return Err(v);
    }

    // if let Err(v) =
    load_gtp_dictionary("src/config/GTPv2_Dictionary.json");
    // {
    //     println!("Failed load dictionary file");
    //     return Err(v);
    // }

    //Create Peer structure based on config_peer file
    let ret = create_peer().await;

    //IP pool
    let ret = prepare_ip_pool().await;


    // let msg_queue = MsgQue::new();

    let recv_handle;
    let config = CONFIG_MAP.read().await;
    let src_port = config.get("SrcPort").clone();
    drop(config);

    if let Some(src_port) = src_port {
        // let src_port = src_port.to_string();
        if let Ok(recv_socket) = socket_create( format!("0.0.0.0:{}", src_port)) {

            println!("Socket Successfully Created!: {:?}", recv_socket);
            // thread::spawn(move || gtpv2_recv_task(recv_socket, session_list, teid_list) );
                // gtpv2_recv_task(&recv_socket, session_list.clone(), teid_list.clone()).await;
            recv_handle = tokio::spawn(async move {
                gtpv2_recv_task(recv_socket).await;
            });
            // dummy(recv_socket);
        }
        else {
            eprintln!("Failed to create socket for address 0.0.0.0:{}", src_port);
        }
    }

    // peer_manage().await;
    let peer_handle = tokio::spawn(async move {
        println!("Peer Manage");
        peer_manage().await;
    });

    let queue_clone = SHARED_QUEUE.clone();
    let queue_handle = tokio::spawn(async move {
        println!("Queue Manage");
        queue_clone.lock().await.check_timer().await;
    });

    let _ = tokio::try_join!( peer_handle, queue_handle);

    Ok(())
}
