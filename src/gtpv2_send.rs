use std::io;
use std::sync::Arc ;
use std::net::UdpSocket;

use crate::gtp_msg::*;
use crate::timer::*;
use crate::peers::*;
use crate::gtpv2_ie::*;



pub fn send_udp_data(data: &[u8], ip: &str, port: u16)
-> io::Result<()> {
    // Create a new UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Binds to an available port on the local machine

    // Format the target address
    let addr = format!("{}:{}", ip, port);

    // Send the data
    socket.send_to(data, addr)?;

    Ok(())
}


pub async fn make_gtpv2(msg_type: u8, body: &[u8;1024],
    oldpeer: Peer, t_flag: bool, len: u8)
-> Result<(), String> {
    let mut peer;
    let mut length :usize = 0;
    let bodylen: u16 = len as u16;
    let mut buffer: [u8; 1024] = [0; 1024]; 

    let result = get_peer(&oldpeer.ip);
    match result {
        Ok(value) => peer = value,
        _ => return  Err ("Fail to get peer".to_string()),
    }

    (_, length) = Gtpv2CHeader::encode (&mut buffer, false,
                        t_flag, false, msg_type,
                        bodylen, oldpeer.teid, peer.tseq, 0) ;

    buffer[length..length + len as usize].copy_from_slice(&body[..len as usize]);

    let pkt = EncapPkt::new(peer.ip, msg_type, buffer[..length + len as usize].to_vec());

    send_udp_data(&buffer[..length+len as usize], &peer.ip.to_string(), peer.port);

    let mut queue_locked = SHARED_QUEUE.lock().await;
    queue_locked.push(pkt);

    peer.update_last_echo_snd_time();
    peer.increase_count();

    Ok(())
}

