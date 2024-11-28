mod gtpv2_type;
mod gtp_msg;
mod config;
mod udpsock;

use gtp_msg::Peer;
use gtpv2_type::{gtpv_msg_type_vals, gtpv_ie_type_vals};
use config::*;


fn main() {
    /* Read Config */
    // _ = read_conf("src/config");
    _ = read_peer("src/config_peer");
    let config = CONFIG_MAP.read().unwrap();

    Peer::print();

    // let socket = socket_create(src_bind);
    // if socket.is_err() {
    //     println!("{}", socket.unwrap_err());
    //     return Err(());
    // }

}
