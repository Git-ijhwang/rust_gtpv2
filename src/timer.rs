use tokio::time::{self, Duration, Instant};
use tokio::sync::{mpsc, oneshot};

use std::net::Ipv4Addr;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::gtp_msg::*;
use crate::gtpv2_type::*;
use crate::peers::*;

// pub async fn start_timer(
// interval: Duration,
// mut rx: mpsc::Receiver<String>)
// {

// let mut timer = time::interval(interval);
// let mut count = 0;

// loop {
//     tokio::select! {
//         Some(signal) = rx.recv() => {

//             println!("<======== 타이머: 외부 종료 신호 수신, 타이머 종료 {}", signal  );
//             break; // 외부 신호를 받으면 종료
//         }
//         _ = timer.tick() => {
//             count += 1;
//             println!("타이머: {}번째 동작 실행", count);
//             // 추가적인 처리 로직 (예: 네트워크 전송 등)
//         }
//     }
// }
// println!("Done.");
// }


// async fn timer_manager(
// timer_map: Arc<Mutex< HashMap< u64, tokio::time::Instant >>>)
// {

// loop {
//     tokio::time::sleep(Duration::from_secs(1)).await; //1초 마다 hashmpa search

//     let now = tokio::time::Instant::now();
//     let mut expired = vec![]; //for expired timer

//     {
//         let mut map = timer_map.lock().unwrap();
//         for (&id, &expiry) in map.iter() {
//             if now >= expiry {
//                 expired.push(id);
//             }
//         }

//         for id in &expired {
//             map.remove(id);
//         }
//     }

//     // 재전송 처리
//     for id in expired {
//         println!("Resending message for ID: {}", id);
//     }
// }
// }


static ECHO_TIMEOUT: Duration = Duration::from_secs(3);
pub struct EncapPkt {
    peer_index: Ipv4Addr,
    msg_type:   u8,
    send_time:  Instant,
    expiry:     Duration,// duration time in Sec
    send_count: u32,
    pub datalen: usize,
    pub data:       [u8; 1024],
}

impl Clone for EncapPkt {
    fn clone(&self) -> Self {
        EncapPkt {
            peer_index: self.peer_index,
            msg_type:   self.msg_type,
            expiry:     self.expiry,
            send_time:  self.send_time,
            send_count: self.send_count,
            datalen:    self.datalen,
            data:       self.data,
        }
    }
}

impl EncapPkt {
    pub fn new(index: Ipv4Addr, msg_type: u8) -> Self {
        EncapPkt {
            peer_index: index,
            msg_type:   msg_type,
            send_time:  Instant::now(),
            expiry:     Duration::new(3, 0),
            send_count: 1,
            datalen:    0,
            data:       [0u8; 1024],
        }
    }

    pub fn copy_data(&mut self, org: &[u8], len:usize ) {
        self.data[..len].copy_from_slice(&org[..len]);
    }

    pub fn put_que (&self, queue: &mut MsgQue) {
        queue.push(self.clone());
    }
}


pub struct MsgQue {
    queue: Arc<Mutex<VecDeque<EncapPkt>>>,
}

impl MsgQue  {
    pub fn new() -> Self {
        MsgQue {
            queue: Arc::new(Mutex::new( VecDeque::new())),
        }
    }

    pub fn rm_peer(&mut self, value: Ipv4Addr) {
        let mut queue = self.queue.lock().unwrap();
        queue.retain(|item| item.peer_index != value);
    }

    pub fn push(&mut self, value: EncapPkt) {
        self.queue.lock().unwrap().push_back(value);
    }

    pub fn pop(&mut self) -> EncapPkt {
        self.queue.lock().unwrap().pop_front().unwrap()
    }

    pub fn que_print(self) {
        let queue = self.queue.lock().unwrap();
        for x in queue.iter() {
            println!("{:?}", x.data);
        }

    }

    pub async fn check_timer (&mut self ) {
        let mut expired = vec![]; //for expired timer

        loop {
            tokio::time::sleep(Duration::from_millis(100)).await; //sleep 100ms

            let now = tokio::time::Instant::now();

            {
                let mut queue = self.queue.lock().unwrap();
                for x in queue.iter_mut() {
            // queue.iter_mut().for_each( |x: &mut EncapPkt| 
                    if now.duration_since(x.send_time) > x.expiry {
                        if x.send_count <= 3  {
                            println!("Resend message");
                            /* TODO: */
                            x.send_time = Instant::now();
                            x.send_count += 1;
                            if x.msg_type == GTPV2C_ECHO_REQ {
                                x.expiry = ECHO_TIMEOUT;
                            }
                        }
                        else {
                            expired.push(x.clone());
                        }
                    }
                }
            }

            for item in &mut expired {
                let mut peer = Peer::get_peer(item.peer_index).unwrap();
                let buf = format!("Before: {}", peer.get_peer_status());
                peer.deactivate_peer_status();
                println!("{} After: {}",buf, peer.get_peer_status());
                self.rm_peer(item.peer_index);
            }

            expired.clear();
        }
    }
}