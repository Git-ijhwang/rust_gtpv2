use std::net::UdpSocket;
use std::net::Ipv4Addr;
use log::{debug, error, info, trace, warn};
use rand::seq;
use std::sync::Arc;
use std::sync::Mutex;

use crate::dump::*;
use crate::peers::*;
use crate::gtp_msg::*;
use crate::gtpv2_type::*;
use crate::pkt_manage::*;
use crate::msg_static::*;
use crate::peers::*;
use crate::session::*;


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


pub async fn build_gtpv2_message( body: &[u8], msg_type: u8, session: Arc<Mutex<Session>>, t_flag: bool, len: u8)
-> Result<Vec<u8>, String>
{
    let bodylen: u16 = len as u16;
    let mut buffer: [u8; 1024] = [0; 1024];

    let peers = GTP2_PEER.lock().await;
	let locked_session = session.lock().unwrap();

	let peer = if let Some(val) = peers.get(&u32::from(locked_session.peer_ip)) {
		val 
	}
	else {
		return Err(("Fail to get peer").to_string());
	};

    let (_, length) = Gtpv2CHeader::encode( &mut buffer,
            false, t_flag, false, msg_type,
            bodylen, peer.teid, peer.tseq, 0);

    buffer[length..length + len as usize].copy_from_slice(&body[..len as usize]);

    print_hex(&buffer, length + len as usize);

    Ok(buffer[..length + len as usize].to_vec())
}


pub async fn send_gtpv2_message( buffer: &[u8], msg_type: u8, session:Arc<Mutex<Session>>, resend_flag: bool)
-> Result<(), String> {

    let locked_session = session.lock().unwrap();
    let mut peers = GTP2_PEER.lock().await;//unwrap();
    let peer = if let Some(value) = peers.get_mut(&u32::from(locked_session.peer_ip)) {
        value
    }
    else {
        return Err(("Fail to get peer").to_string());
    };

    if resend_flag {
        info!("Encap the packet");
        let pkt = EncapPkt::new(peer.ip, msg_type, buffer.to_vec());
        let mut queue_locked = SHARED_QUEUE.lock().await;
        queue_locked.push(pkt);
        drop(queue_locked);
    }

    let ret = send_udp_data(buffer, &peer.ip.to_string(), peer.port);
    match ret {
        Ok(v) => {
            if v <= 0 {
                return Err("Fail to send message to peer".to_string());
            }

            update_peer(peer);
            Ok(())
        }
        _ => Err("Fail to send message to peer".to_string()),
    }
}


pub async fn build_gtpv2_message_without_session (body: &[u8], msg_type: u8, seq: u32, len: u8)
-> Result<Vec<u8>, String>
{
    let bodylen: u16 = len as u16;
    let mut buffer: [u8; 1024] = [0; 1024];

    let (_, length) = Gtpv2CHeader::encode( &mut buffer,
            false, false, false, msg_type,
            bodylen, 0, seq, 0);

    buffer[length..length + len as usize].copy_from_slice(&body[..len as usize]);

    print_hex(&buffer, length + len as usize);

    Ok(buffer[..length + len as usize].to_vec())
}


pub async fn send_gtpv2_message_without_session( buffer: &[u8], msg_type: u8, peer: &mut Peer, resend_flag: bool)
-> Result<(), String> {

    if resend_flag {
        info!("Encap the packet");
        let pkt = EncapPkt::new(peer.ip, msg_type, buffer.to_vec());
        let mut queue_locked = SHARED_QUEUE.lock().await;
        queue_locked.push(pkt);
        drop(queue_locked);
    }

    let ret = send_udp_data(buffer, &peer.ip.to_string(), peer.port);
    match ret {
        Ok(v) => {
            if v <= 0 {
                return Err("Fail to send message to peer".to_string());
            }

            update_peer(peer);
            update_message_stats(msg_type, true, 0);
            Ok(())
        }
        _ => Err("Fail to send message to peer".to_string()),
    }
}


// fn create_session_reject (peer:Peer, session:Arc<Mutex<Session>>)
// {
//     let mut buffer:[u8;1024] = [0u8;1024];
//     let session = session.lock().unwrap();

//     trace!("Create Session Reject ");
//     // make_gtpv2( buffer, GTPV2C_CREATE_SESSION_RSP,
//     //         peer.ip.clone(), true, total_len as u8, false).await;
//     // let buf = build_gtpv2_message(buffer, GTPV2C_CREATE_SESSION_RSP, true, total_len as u8, peer)
//     // send_gtpv2_message(buf, GTPV2C_CREATE_SESSION_RSP, peer, false);

//     let buf;

//     match build_gtpv2_message(&buffer, GTPV2C_MODIFY_BEARER_RSP,
//         session.clone(), true, total_len as u8 ).await {
//         Ok(v) => buf = v,
//         Err(_) => return Err("Fail to make header".to_string()),
//     }
//     send_gtpv2_message(&buf, GTPV2C_MODIFY_BEARER_RSP, session.clone(), false).await;

// }