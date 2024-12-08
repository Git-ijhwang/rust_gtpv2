use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use core::result::Result;
use std::thread;

use crate::gtp_dictionary::{self, *};
use crate::validate_gtpv2::*;
use crate::gtpv2_type::*;
use std::io::Error;
use crate::peers::*;

const BUFSIZ: usize = 8192;

#[derive(Debug)]
struct Gtpv2Peer {
    ipaddr: u32,
    status: u8,
}

// #[derive(Debug, Clone, Copy)]
struct Gtpv2IeDetailInfo {
    ie_type:        u8,
    ie_min_length:  u8,
    ie_max_length:  u8,
    reserved:       u8,
    ie_instance:    u8,
    ie_presence:    u8,
    group_ie_info:  Option<Box<Gtpv2IeInfo>>, // Pointer to group IE info, optional to handle null pointers
}

#[derive(Debug, Clone, Copy)]
struct Gtpv2IeInfo {
    msg_type:       u8,     // Message Type
    mie_count:      usize,  // Mandatory IE Count
    // ie_parse_info: Vec<Vec<IeParseDetails>>,
    // ie_detail_info: HashMap<(u8, u8), Gtpv2IeDetailInfo>,
}

impl Gtpv2IeInfo {
    fn new (msg_type: u8) -> Self {
        Self {
            msg_type,
            mie_count:      0,
            // ie_detail_info: HashMap::new(),
        }
    }

    fn add_ie_parse_info(&mut self, msg_type: u8,
        min_len: u8, max_len: u8, received: u8,
        ie_instance: u8, ie_presence: u8,
        // gie_info: &[Gtpv2IeInfo]
    ) {
            // if gie.ie_type == 0 {
            //     break; // End of array
            // }

            // let key = (gie.ie_type, gie.ie_instance);
            // let parse_info = Gtpv2IeParseInfo {
            //     ie_min_length: gie.ie_min_length,
            //     ie_max_length: gie.ie_max_length,
            //     ie_presence: gie.ie_presence,
            //     reserved: gie.reserved,
            //     group_ie_info: None,
            // };

            // self.ie_parse_info.insert(key, parse_info);

            // if gie.ie_presence == GTPV2C_IE_PRESENCE_MANDATORY {
            //     self.mie_count += 1;
            // }
        }
}


#[derive(Debug)]
struct IeParseDetails { 
    ie_min_length: u8,
    ie_max_length: u8,
    ie_presence: u8,
    reserved: u8,
    group_ie_info: Option<Box<Gtpv2GroupIeParseInfo>>,
}

#[derive(Debug)]
struct IeParseGroupDetails { //XXX
    ie_min_length: u8,
    ie_max_length: u8,
    ie_presence: u8,
    reserved: u8,
}


#[derive(Debug)]
struct Gtpv2GroupIeParseInfo {//gtpv2_group_ie_parse_info_t
    group_ie_type: u8,
    mie_count: usize,

    ie_parse_info: Vec<Vec<IeParseDetails>>,//[typemax][instancemax]
}


#[derive(Debug)]
struct Gtpv2IeParseInfo { //gtpv2_ie_parse_info_t
    group_ie_type: u8,
    mie_count: usize,

    ie_parse_info: Vec<Vec<IeParseDetails>>,//[typemax][instancemax]
}


#[derive(Debug)]
struct Gtpv2Msg {
    msg_type: u8,
    msg_buf: Vec<u8>,
    teid_present: bool,
    valid_ie: Vec<Vec<bool>>,
    pie: Vec<Vec<Option<Vec<u8>>>>,
}

fn get_gtpv2_peer(ip: u32) -> Option<Arc<Mutex<Gtpv2Peer>>> {
    // Example function to get a peer (to be implemented)
    // Return None if no peer is found
    None
}

fn send_gtpv2_version_not_supported(
    _peer: Arc<Mutex<Gtpv2Peer>>,
    _peerip: u32,
    _peerport: u16,
    _seqnum: u32,
) {
    // Example function to send version not supported message
    println!("Sent version not supported message.");
}


fn is_group_ie(ie_type: u8, dictionary: &GtpMessage) -> bool {
    dictionary.ie_list
        .iter()
        .find(|info| info.ie_type == ie_type )
        .map_or(false, |info| info.group_ie_info.is_some())
}

pub fn find_dictionary(msg_type: u8) -> Result<GtpMessage, String> {

    let dictionary = GTP_DICTIONARY.read().unwrap();

    let result = dictionary 
            .iter()
            .find(|msg| msg.msg_type == msg_type )
            .ok_or(format!("Unknown message type: 0x{:x}", msg_type));
    
    match result {
        Ok(data) =>  Ok(data.clone()), 
        Err(err) =>  Err("Unknow Message type".to_string()),
    }
}

fn extract_nested_ies(raw_data: &[u8], dictionary: &GtpMessage) -> Vec<(u8, usize) > {
    let mut ies = Vec::new();
    let mut index = 0;

    while index + 4 <= raw_data.len() {
        let ie_type = raw_data[index];
        let ie_length = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]) as usize;

        if index + 4 + ie_length > raw_data.len() {
            break;
        }

        ies.push((ie_type, ie_length ));
        index += 4 + ie_length;
    }

    ies
}


fn extract_ies(raw_data: &[u8], dictionary: &GtpMessage) -> Vec<(u8, usize, Vec<(u8, usize)>) > {
    let mut ies = Vec::new();
    let mut index = 0;

    while index + 4 <= raw_data.len() {
        let ie_type = raw_data[index];
        let ie_length = u16::from_be_bytes([raw_data[index + 1], raw_data[index + 2]]) as usize;

        if index + 4 + ie_length > raw_data.len() {
            break;
        }

        let ie_value = &raw_data[index+4..index+4+ie_length];
        let nested_ies = if is_group_ie(ie_type, dictionary) {
            extract_nested_ies(ie_value, dictionary)
        }
        else {
            Vec::new()
        };

        ies.push((ie_type, ie_length, nested_ies));
        index += 4 + ie_length;
    }

    ies
}


pub fn recv_gtpv2( _data: &[u8],
    // _peer: Arc<Mutex<Gtpv2Peer>>,
    peer: Peer,
    // _n: usize,
    // _peerip: u32,
    // _peerport: u16,
) {
    // Example function to handle received GTPv2 data
    println!("Processed GTPv2 data.");
    
    let raw_message: Vec<u8> = vec![ ];
    let hdr_len;
    let rcv_msg_type;
    let msg_define;

    match parse_header(raw_message.clone()) {
        Ok( (msg_type, n)) => {
            rcv_msg_type=msg_type; hdr_len = n; },
        Err(s) => {
            println!("{}", s); return; },
     }

    let dictionary = GTP_DICTIONARY.read().unwrap();

    let result = dictionary 
            .iter()
            .find(|msg| msg.msg_type == rcv_msg_type )
            .ok_or(format!("Unknown message type: 0x{:x}", rcv_msg_type));
    
    match result {
        Ok(data) => msg_define = data, 
        Err(err) => { println!("{}", err); return },
    }

    let ies = extract_ies(&raw_message[hdr_len..], msg_define);
    if let Err(ret) = validate(&ies, msg_define) {
        println!("Validation Check false: [{}]", ret);
    }

    //validation check완료.
    // TODO:
 
}


/*
    {
        let peer = peer.lock().unwrap();
        if peer.status == 0
            && msg_type != GTPV2C_ECHO_REQ
            && msg_type != GTPV2C_ECHO_RSP
        {
            println!( "(GTPv2-RECV) Peer ({}) is not Active", Ipv4Addr::from(peer.ipaddr));
            return ;
        }
    }

*/



pub fn gtpv2_recv_task(socket: UdpSocket) {
    let mut buf = [0u8; BUFSIZ];
    let timeout = Duration::from_millis(1000);

    loop {
        println!("Start Recv Task");

        socket.set_read_timeout(Some(timeout)).unwrap();

        match socket.recv_from(&mut buf) {
            Ok((nbyte, addr)) => {
                if let SocketAddr::V4(addr) = addr {
                    let sin_addr = addr.ip();
                    let sin_port = addr.port();

                    match get_peer(sin_addr) {
                        Err(_) => {
                            println!("This is not registered PEER!");
                        } 

                        Ok(peer) => {
                            // if let Err(_) =
                            _ = recv_gtpv2(&buf[..nbyte], peer);
                        }
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout occurred, continue to next iteration
                continue;
            }
            Err(e) => {
                println!("Error receiving data: {:?}", e);
                break;
            }
        }
    }
}


// fn main() {
//     let socket = Arc::new(
//         UdpSocket::bind("0.0.0.0:2123")
//             .expect("Could not bind to address"),
//     );

//     let recv_socket = socket.clone();
//     thread::spawn(move || gtpv2_recv_task(recv_socket));

//     loop {
//         // Main thread can perform other tasks
//     }
// }
