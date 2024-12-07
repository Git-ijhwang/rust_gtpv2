mod gtpv2_type;
mod gtpv2_send;
mod gtp_msg;
mod config;
mod udpsock;
mod packet;
mod timer;
mod gtp_dictionary;
mod recv_gtpv2;
mod validate_gtpv2;
mod peers;

use config::*;
use gtp_msg::*;
use std::thread;
use std::io::Error;
use core::result::Result;
use tokio::time::{self, Duration};
use crate::peers::*;
use crate::udpsock::*;
use crate::recv_gtpv2::*;
use crate::gtp_dictionary::*;


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

    load_gtp_dictionary("src/config/GTPv2_Dictionary.json");

    create_peer();

    let bind_addr = CONFIG_MAP.read().unwrap();

    if let Ok(recv_socket) = socket_create( format!("0.0.0.0:{}", bind_addr.get("SrcPort").unwrap().to_string())) {
        println!("Socket Successfully Created!: {:?}", recv_socket);
        thread::spawn(move || gtpv2_recv_task(recv_socket));
        // recv_gtpv2();
    }
    else{
        eprintln!("Failed to create socket for address 0.0.0.0:{}",
        bind_addr.get("SrcPort").unwrap());
    }

    loop { tokio::time::sleep(Duration::from_secs(1)).await; };

}
