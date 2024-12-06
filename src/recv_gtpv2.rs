use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use crate::gtp_dictionary::{self, *};
use crate::validate_gtpv2::*;
use crate::gtpv2_type::*;

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
        .find(|info| info.ie_type == ie_type)
        .map_or(false, |info| info.group_ie_info.is_some())
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



pub fn recv_gtpv2(
    // _peer: Arc<Mutex<Gtpv2Peer>>,
    // _data: &[u8],
    // _n: usize,
    // _peerip: u32,
    // _peerport: u16,
) {
    // Example function to handle received GTPv2 data
    println!("Processed GTPv2 data.");
    
    // msg_alloc 호출
    // gtpv2_ie_parse 호출


    let raw_message: Vec<u8> = vec!
        [ 0x48, 0x20, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00,   0x02, 0x4a, 0x20, 0x00,
                                                                                    0x01, 0x00, 0x08, 0x00,
          0x54, 0x00, 0x16, 0x10, 0x11, 0x05, 0x00, 0xf7,   
                                                            0x4c, 0x00, 0x06, 0x00, 0x28, 0x01, 0x11, 0x51,
          0x00, 0x70,
                      0x4b, 0x00, 0x0f, 0x00, 0x31, 0x32,   0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61,
          0x62, 0x63, 0x64, 0x65, 0x66,
                                        0x56, 0x00, 0x0d,   0x00, 0x18, 0x54, 0xf0, 0x60, 0x04, 0xd2, 0x54,
          0xf0, 0x60, 0x00, 0xbc, 0x61, 0x4e,
                                              0x53, 0x00,   0x03, 0x00, 0x54, 0xf0, 0x60,
                                                                                          0x52, 0x00, 0x01,
          0x00, 0x06,
                      0x4d, 0x00, 0x03, 0x00, 0x08, 0x00,   0x00,
                                                                  0x57, 0x00, 0x09, 0x00, 0x8a, 0x10, 0x00,
          0xc3, 0x56, 0x0a, 0x0a, 0x01, 0x47,
                                              0x57, 0x00,   0x09, 0x01, 0x87, 0x00, 0x00, 0x00, 0x00, 0x0a,
          0x0a, 0x03, 0x49,
                            0x47, 0x00, 0x29, 0x00, 0x69,   0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x2e,
          0x6c, 0x67, 0x75, 0x70, 0x6c, 0x75, 0x73, 0x2e,   0x63, 0x6f, 0x2e, 0x6b, 0x72, 0x2e, 0x6d, 0x6e,
          0x63, 0x30, 0x30, 0x36, 0x2e, 0x6d, 0x63, 0x63,   0x34, 0x35, 0x30, 0x2e, 0x67, 0x70, 0x72, 0x73,
          
          0x80, 0x00, 0x01, 0x00, 0x00,
                                        0x63, 0x00, 0x01,   0x00, 0x01,
                                                                        0x4f, 0x00, 0x05, 0x00, 0x01, 0x00,
          0x00, 0x00, 0x00,
                            0x7f, 0x00, 0x01, 0x00, 0x00,   
                                                            0x48, 0x00, 0x08, 0x00, 0x00, 0x01, 0x86, 0xa0,
          0x00, 0x01, 0x86, 0xa0,       
                                  0x4e, 0x00, 0x1d, 0x00,   0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10,
          0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06,   0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,
          0x0d, 0x00, 0x00, 0x0a, 0x00,
                                        0x5d, 0x00, 0x1f,   0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00,
          0x16, 0x00, 0x04, 0x06, 0x00, 0x00, 0x00, 0x00,   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,   
                                                            0x72, 0x00, 0x02, 0x00, 0x63, 0x00, 0x5f, 0x00,
          0x02, 0x00, 0x08, 0x00];

    let mut rcv_msg_type;
    let hdr_len;
    match parse_header(raw_message.clone()) {
        Ok( (msg_type, n)) => {
            rcv_msg_type=msg_type; hdr_len = n; },
        Err(s) => {
            println!("{}", s); return; },
     }

    let dictionary = GTP_DICTIONARY.read().unwrap();
    let mut msg_define;
    let result = dictionary 
            .iter()
            .find(|msg| msg.msg_type == rcv_msg_type)
            .ok_or(format!("Unknown message type: 0x{:x}", rcv_msg_type));
    
    match result {
        Ok(data) =>  msg_define = data, 
        Err(err) =>{ println!("{}", err); return},
    }


    let ies = extract_ies(&raw_message[hdr_len..], msg_define);

    let ret = validate(&ies, &dictionary, rcv_msg_type, msg_define);
    // let ret = validate(&ies, &GTP_DICTIONARY.read().unwrap(), rcv_msg_type, msg_define);
    if ret == false {
        println!("Validation Check false");
    }

	// parse_gtp_message(&raw_message, &dictionary);
    // __proc_gtpv2 호출

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
    let timeout = Duration::from_secs(1);

    loop {
        socket.set_read_timeout(Some(timeout)).unwrap();

        // match socket.recv_from(&mut buf) {
        //     Ok((nbyte, addr)) => {
        //         if let SocketAddrV4::V4(addr) = addr {
        //             let sin_addr = addr.ip();
        //             let sin_port = addr.port();
        //             __recv_gtpv2(&buf[..nbyte], *sin_addr, sin_port);
        //         }
        //     }
        //     Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
        //         // Timeout occurred, continue to next iteration
        //         continue;
        //     }
        //     Err(e) => {
        //         println!("Error receiving data: {:?}", e);
        //         break;
        //     }
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
