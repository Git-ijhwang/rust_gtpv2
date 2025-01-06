use tokio::time::*;
use std::sync::Arc ;
use tokio::sync::Mutex;
use std::net::Ipv4Addr;
use std::collections::VecDeque;
use log::{debug, error, info, trace, warn};

use crate::gtpv2_type::*;
use crate::peers::*;


static ECHO_TIMEOUT: Duration = Duration::from_secs(5);
pub struct EncapPkt {
    pub peer_index: Ipv4Addr,
    pub msg_type:   u8,
    pub seq:        u32,
    pub send_time:  Instant,
    pub expiry:     Duration,   // duration time in Sec
    pub send_count: u32,
    pub datalen:    usize,
    pub data:       Arc<[u8]>,
}


impl Clone for EncapPkt {
    fn clone(&self) -> Self {
        EncapPkt {
            peer_index: self.peer_index,
            msg_type:   self.msg_type,
            seq:        self.seq,
            expiry:     self.expiry,
            send_time:  self.send_time,
            send_count: self.send_count,
            datalen:    self.datalen,
            data:       Arc::clone(&self.data),
        }
    }
}


impl EncapPkt {
    pub fn new(index: Ipv4Addr, msg_type: u8, data: Vec<u8>) -> Self {
        let datalen = data.len();
        EncapPkt {
            peer_index: index,
            msg_type,
            seq:        0,
            send_time:  Instant::now(),
            expiry:     Duration::new(3, 0),
            send_count: 1,
            datalen,
            data:       Arc::from(data.into_boxed_slice()),
        }
    }

    // pub fn copy_data(&mut self, org: &[u8], len:usize ) {
    //     self.data[..len].copy_from_slice(&org[..len]);
    // }

    // pub fn put_que (&self, queue: &mut MsgQue) {
    //     queue.push(self.clone());
    // }

    pub fn update_send_time(&mut self) {
        self.send_time = Instant::now();
    }

    pub fn increment_send_count(&mut self) {
        self.send_count += 1;
    }

    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.send_time) > self.expiry
    }
}


lazy_static::lazy_static! {
    pub static ref SHARED_QUEUE: Arc<Mutex<MsgQue>> = Arc::new(Mutex::new(MsgQue::new()));
}


#[derive(Clone )]
pub struct MsgQue {
    queue: VecDeque<EncapPkt>,
}

impl MsgQue  {
    pub fn new() -> Self {
        MsgQue {
            queue: VecDeque::new(),
        }
    }

    pub fn rm_peer(&mut self, value: Ipv4Addr) {
        self.queue.retain(|item| item.peer_index != value );
    }
    pub fn rm_pkt(&mut self, value: Ipv4Addr, seq: u32) {
        self.queue.retain(|item| item.peer_index != value || item.seq != seq);
    }

    pub fn push(&mut self, value: EncapPkt) {
        self.queue.push_back(value);
    }

    pub fn pop(&mut self) -> EncapPkt {
        self.queue.pop_front().unwrap()
    }

     pub fn que_print(self) {
        // let queue = self.queue;
        for x in self.queue {
            println!("{:?}", x.data);
        }
    }
}


pub async fn find_pkt_in_queue(peer_index: Ipv4Addr, rcv_seq: u32) {
    let mut queue = SHARED_QUEUE.lock().await;
    match queue.queue.iter().find(|x| x.peer_index == peer_index && x.seq == rcv_seq) {
        Some(pkt) => {
            info!("Found Packet in Queue");
            queue.rm_pkt(peer_index, rcv_seq);
        }
        None => {
            error!("Can't find Packet in Queue");
        }
    }
}

pub async fn check_timer () {

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await; //sleep 100ms
    
        let mut expired = vec![];
        let now = tokio::time::Instant::now();

		let mut queue = SHARED_QUEUE.lock().await;
		queue.queue.retain_mut(|x| {

			if now.duration_since(x.send_time) > x.expiry {

				if x.send_count <= 3 {
					x.send_time = tokio::time::Instant::now();
					x.send_count += 1;

					if x.msg_type == GTPV2C_ECHO_REQ {
						x.expiry = ECHO_TIMEOUT;
					}
					if x.msg_type == GTPV2C_CREATE_SESSION_REQ {
						x.expiry = ECHO_TIMEOUT;
					}
					info!("Msg type: {}", x.msg_type);

					true // Keep the item
				} else {
					expired.push(x.clone());
					error!("Expire!!!");
					false // Remove from Queue
				}
			} else {
				true // Keep the item
			}
		});
		drop(queue);

        let mut list = GTP2_PEER.lock().await;//unwrap();
        for item in &mut expired {
            
            let key = u32::from(item.peer_index);//.into();
            if let Some(peer) = list.get_mut(&key) {
                    peer.deactivate_peer_status();
            }
            else {
                error!("Fail to get peer for {}", item.peer_index);
            }

            SHARED_QUEUE.lock().await.rm_peer(item.peer_index);
        }
		drop(list);

        expired.clear();
    }
}