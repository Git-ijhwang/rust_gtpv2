use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use crate::gtp_dictionary::{self, *};
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

    fn add_ie_parse_info(&mut self,
        msg_type: u8,
        min_len: u8,
        max_len: u8,
        received: u8,
        ie_instance: u8,
        ie_presence: u8,
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

    // pub fn add_ie_parse_info (&mut self,
    //     ie_info: &Gtpv2IeInfo
    // ) {

    // }
// }


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



// fn gtpv2_ie_parse(hmsg: &[u8], parse_info: &Gtpv2IeParseInfo) -> Result<(), String> {
//     let mut pmsg = Gtpv2Msg {
//         msg_type: hmsg[0],
//         msg_buf: hmsg.to_vec(),
//         teid_present: hmsg.len() > 8,
//         valid_ie: vec![vec![false; 16]; 256],
//         pie: vec![vec![None; 1024]; 256],
//     };

//     let mut ie_start = if pmsg.teid_present {
//         &hmsg[12..]
//     } else {
//         &hmsg[8..]
//     };

//     let ie_end = hmsg.len();
//     let mut ie_count = 0;
//     let mut mie_count = 0;

//     while ie_start.len() >= 4 {
//         let ie = Gtpv2cIeTlv {
//             t: ie_start[0],
//             l: u16::from_be_bytes([ie_start[1], ie_start[2]]),
//             i: ie_start[3],
//         };
//         let ie_length = ie.l as usize;

//         if ie.t as usize >= parse_info.ie_parse_info.len() //GTPV2_IE_TYPE_MAX
//             || (ie.i & 0x0F) as usize >= parse_info.ie_parse_info[0].len() {//GTPV2_IE_INSTANCE_MAX
//                 //perr->cause GTPV2C_CAUSE_INVALID_MESSAGE_FORMAT;
//                 return Err(format!(
//                 "Invalid IE type {} and instance {}",
//                 ie.t, ie.i & 0x0F
//             ));
//         }

//         if ie_count == 1024 { //GTPV2C_IE_MAX
//             //perr->cause GTPV2C_CAUSE_REQUEST_REJECTED;
//             return Err("Too many IEs".to_string());
//         }

//         if ie_start.len() < 4 + ie_length {
//             //perr->cause GTPV2C_CAUSE_INVALID_LENGTH;
//             return Err(format!(
//                 "Invalid length for IE type {} instance {}",
//                 ie.t, ie.i & 0x0F
//             ));
//         }

//         let details = &parse_info.ie_parse_info[ie.t as usize][(ie.i & 0x0F) as usize];
//         if details.ie_presence == 0 {
//             println!(
//                 "Unexpected IE type {} instance {} length {}",
//                 ie.t, ie.i & 0x0F, ie_length
//             );
//             continue;
//         }

//         if ie_length < details.ie_min_length as usize || ie_length > details.ie_max_length as usize {
//             //perr->cause GTPV2C_CAUSE_INVALID_LENGTH;
//             return Err(format!(
//                 "IE type {} instance {} incorrect length {}",
//                 ie.t, ie.i & 0x0F, ie_length
//             ));
//         }

//         if let Some(group_info) = &details.group_ie_info {
//             gtpv2_group_ie_parse(&mut pmsg, ie_count, group_info, &ie_start[4..4 + ie_length])?;
//         }

//         if details.ie_presence == 1 {
//             mie_count += 1;
//         }

//         ie_count += 1;
//         ie_start = &ie_start[(4 + ie_length)..];
//     } //end while

//     if parse_info.mie_count != mie_count {
//         return Err("Mandatory IE missing".to_string());
//     }

//     Ok(())
// }

// 단일 IE 및 중첩된 그룹 검증
fn validate_ie(
    expected_ie: &IeInfo, ies: &[(u8, usize, Vec<(u8, usize)>)], is_top_level: bool
) -> Result<(), String> {

    println!("{:?}--", ies);
    let mut matches =
            ies.iter().filter(|(ie_type, _, _)| *ie_type == expected_ie.ie_type);

    println!("{:?}", matches);
    match expected_ie.presence.as_str() {
        "MANDATORY" if matches.next().is_none() => {
            return Err(format!("Mandatory IE {}(0x{:x}) is missing",expected_ie.ie_type, expected_ie.ie_type)); 
        }
        "CONTIDIONAL" | "CONTIDIONAL_OPTION" | "OPTIONAL" => {}
        _ => (),
    }

    // 중첩 그룹 검증
    if let Some(group_info) = &expected_ie.group_ie_info {
        if let Some((_, _, nested_ies)) = ies
                            .iter()
                            .find(|(ie_type, _, _)| *ie_type == expected_ie.ie_type){
                                for group_ie in group_info {
                                    validate_ie(group_ie, nested_ies, false)?;
                                }
                            }
                            else {
                                return Err(format! (
                                    "Mandatory Group IE {}(0x{:x}) is missing",
                                    expected_ie.ie_type, expected_ie.ie_type
                                ));
                            }
        for group_ie in group_info {
            validate_ie(group_ie, ies, false)?;
        }
    }

    Ok(())
}


fn validate_message( msg_type: u8, ies: Vec<(u8, usize, Vec<(u8, usize)>)>, dictionary: &[GtpMessage]) -> Result<(), String> {
    let msg_def = dictionary
        .iter()
        .find(|msg| msg.msg_type == msg_type)
        .ok_or(format!("Unknown message type: {}", msg_type))?;

    println!("{:#?}", msg_def.ie_list);
    for expected_ie in &msg_def.ie_list {
        validate_ie(expected_ie, &ies, true)?;
    }

    Ok(())
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

fn
parse_header (data: Vec<u8>)  -> (u8, usize)
{
    const GTP_VERSION: u8 = 2;
    const GTPV2C_ECHO_REQ: u8 = 1; // Placeholder
    const GTPV2C_ECHO_RSP: u8 = 2; // Placeholder
    let mut p : usize = 0;

    let version = (data[p] & 0xE0) >> 5;
    let pflag = (data[p] & GTPV2_P_FLAG) != 0;
    let tflag = (data[p] & GTPV2_T_FLAG) != 0; p+=1;
    let msg_type = data[p]; p+=1;
    let msg_len = u16::from_be_bytes([data[p], data[p+1]]) as usize; p+=2;
    let mut seqnum;

    const GTPV2_P_FLAG: u8 = 0x10;
    const GTPV2_T_FLAG: u8 = 0x08;

    if tflag {
        p+=4;
    };
    seqnum = u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]) >> 8;
    p+=4;
    // else {
    //     u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]) >> 8
    // };
        return (msg_type, p);

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

    let dictionary = load_gtp_dictionary("GTPv2_Dictionary.json");
    // println!("{:#?}", dictionary);

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

 
    
    let (msg_type, n) = parse_header(raw_message.clone());

    // let msg_type = raw_message[1];
    // let version = (data[p] & 0xE0) >> 5;

    // Search the Message from Dictionary
    let msg_define = dictionary
            .iter()
            .find(|msg| msg.msg_type == msg_type)
            .ok_or(format!("Unknown message type: {:x}", msg_type));

    // println!("MSG DEF: {:#?}. ==== END", msg_define);
    let ies = extract_ies(&raw_message[n..], msg_define.unwrap().clone());
    // Type and Length Tuples for IEs

    match validate_message(msg_type, ies.clone(), &dictionary) {
        Ok(()) => println!("Valid"),
        Err(e) => println!("Error {}", e),
    }


    // let mut Ie_Map = IEMap::new();
    let Ie_Map = make_ie_type_map();

    let msg_dictionary = msg_define.unwrap().clone();
    // Validate IE lengths
    for (ie_type, ie_length) in ies {
        if let Some(ie_def) = msg_dictionary.ie_list.iter().find(|def| def.ie_type == ie_type) {
            if ie_length < ie_def.min_length || ie_length > ie_def.max_length {
                println!(
                    "IE {} [{}] length {} is out of bounds (min: {}, max: {})",
                    Ie_Map.ie_type_to_string(ie_type), ie_type,
                    ie_length, ie_def.min_length, ie_def.max_length
                );
            }
        }
    }

	// parse_gtp_message(&raw_message, &dictionary);
    // __proc_gtpv2 호출

}


/*
fn __recv_gtpv2(data: &[u8], sin_addr: Ipv4Addr, sin_port: u16) 
// -> i32
{

    let peerip = u32::from_be_bytes(sin_addr.octets());
    let peerport = sin_port;
    let seqnum;

    const GTP_VERSION: u8 = 2;
    const GTPV2C_ECHO_REQ: u8 = 1; // Placeholder
    const GTPV2C_ECHO_RSP: u8 = 2; // Placeholder
    let p : usize = 0;

    let version = (data[p] & 0xE0) >> 5;
    let pflag = (data[p] & GTPV2_P_FLAG) != 0;
    let tflag = (data[p] & GTPV2_T_FLAG) != 0; p+=1;
    let msg_type = data[p]; p+=1;
    let msg_len = u16::from_be_bytes([data[p], data[p+1]]) as usize; p+=2;

    const GTPV2_P_FLAG: u8 = 0x10;
    const GTPV2_T_FLAG: u8 = 0x08;

    if tflag {
        p+=4;
    };
    seqnum = u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]) >> 8;
    p+=4;
    // else {
    //     u32::from_be_bytes([data[p], data[p+1], data[p+2], 0]) >> 8
    // };

    // Peer management logic (commented out as requested)
    let peer = match get_gtpv2_peer(peerip) {
        Some(peer) => peer,
        None => {
            println!("(GTPv2-RECV) unknown Peer ({}). Discard.", sin_addr);
            return ;
        }
    };

    if data.len() < 8 {
        println!("(GTPv2-RECV) message too small! Discard.");
        return ;
    }

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

    if msg_len + 4 != data.len() {
        println!(
            "(GTPv2-RECV) Length error (received: {}, expected: {}). Discard.",
            data.len(),
            msg_len + 4
        );
        return ;
    }

    if version != GTP_VERSION {
        println!(
            "(GTPv2-RECV) unsupported GTP version ({}) message! Discard.",
            version
        );
        send_gtpv2_version_not_supported(peer, peerip, peerport, seqnum);
        return ;
    }

    // Receive logic
    // recv_gtpv2(peer, data, data.len(), peerip, peerport)
    recv_gtpv2();
}
*/



fn gtpv2_recv_task(socket: Arc<UdpSocket>) {
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
