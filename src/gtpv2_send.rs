use std::net::UdpSocket;
use std::net::Ipv4Addr;
use log::{debug, error, info, trace, warn};

use crate::dump::*;
use crate::peers::*;
use crate::gtp_msg::*;
use crate::pkt_manage::*;


pub fn send_udp_data(data: &[u8], ip: &str, port: u16)
-> Result<usize, String> {
    // Create a new UDP socket
    // Binds to an available port on the local machine
    let socket;
    let ret = UdpSocket::bind("0.0.0.0:0");
    match ret {
        Ok(v) => socket = v,
        _ => return Err ("Socket Open Error occoured".to_string()),
    }

    // Format the target address
    let addr = format!("{}:{}", ip, port);

    // Send the data
    let ret = socket.send_to(data, addr);
    match ret {
        Ok(v) => Ok(v),
        _ => Err ("Send Error occoured".to_string()),
    }
}


pub async fn make_gtpv2( body: [u8;1024], msg_type: u8,
    ip: Ipv4Addr, t_flag: bool, len: u8, resend_flag: bool)
-> Result<(), String> {
    let mut peer;
    let bodylen: u16 = len as u16;
    let mut buffer: [u8; 1024] = [0; 1024]; 

    trace!("Make GTP Message");
    let mut list = GTP2_PEER.lock().await;//unwrap();
    let key = u32::from(ip);//.into();
    if let Some(value) = list.get_mut(&key) {
        peer = value;
    }
    else {
    drop(list);
        return Err(("Fail to get peer").to_string());
    }
    
    let (_, length) = Gtpv2CHeader::encode (&mut buffer,
        false, t_flag, false, msg_type,
                        bodylen, peer.teid, peer.tseq, 0) ;

    buffer[length..length + len as usize].copy_from_slice(&body[..len as usize]);

    print_hex(&buffer, length+len as usize);

    if resend_flag == true {
        info!("Encap the packet ");
        //Only for Test
    }

    let ret = send_udp_data(&buffer[..length+len as usize], &peer.ip.to_string(), peer.port);
    match ret {
        Ok(v) => {
            if v <= 0 {
                println!("UDP Send failed");
    drop(list);
                return  Err ("Fail to send message to peer".to_string());
            }

            println!("UDP Send Success");
            let pkt = EncapPkt::new(peer.ip, msg_type, buffer[..length + len as usize].to_vec());
            let mut queue_locked = SHARED_QUEUE.lock().await;
            queue_locked.push(pkt);

            drop(queue_locked);
            update_peer(peer);

        },
        _ => {
    drop(list);
            return  Err ("Fail to send message to peer".to_string());
        }
    }

    drop(list);
    Ok(())
}

