use crate::gtpv2_send;
use crate::gtpv2_type::GTPV2C_ECHO_REQ;
use tokio::time::{self, Duration};
use tokio::sync::{mpsc, oneshot};
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;


use tokio::task;
// use timer::start_timer;

pub async fn start_timer(
    interval: Duration,
    mut rx: mpsc::Receiver<String>)
    {

    let mut timer = time::interval(interval);
    let mut count = 0;

    loop {
        tokio::select! {
            Some(signal) = rx.recv() => {

                println!("<======== 타이머: 외부 종료 신호 수신, 타이머 종료 {}", signal  );
                break; // 외부 신호를 받으면 종료
            }
            _ = timer.tick() => {
                count += 1;
                println!("타이머: {}번째 동작 실행", count);
                // 추가적인 처리 로직 (예: 네트워크 전송 등)
            }
        }
    }
    println!("Done.");
}


async fn timer_manager(
    timer_map: Arc<Mutex<
        HashMap< u64, tokio::time::Instant >
        >>)
{

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await; //1초 마다 hashmpa search

        let now = tokio::time::Instant::now();
        let mut expired = vec![]; //for expired timer

        {
            let mut map = timer_map.lock().unwrap();
            for (&id, &expiry) in map.iter() {
                if now >= expiry {
                    expired.push(id);
                }
            }

            for id in &expired {
                map.remove(id);
            }
        }

        // 재전송 처리
        for id in expired {
            println!("Resending message for ID: {}", id);
        }
    }
}

const ECHO_TIMEOUT:u8 =3;
struct EncapPkt {
    msg_type: u8,
    send_time: f64,
    expiry:u8  ,// duration time in Sec
    send_count: u32,
    data: [u8;1024],
}

impl Clone for EncapPkt {
    fn clone(&self) -> Self {
        EncapPkt {
            msg_type : self.msg_type,
            send_time: self.send_time,
            expiry: self.expiry,
            send_count: self.send_count,
            data: self.data,
        }
    }
}

impl EncapPkt {
    fn new() -> Self {
        EncapPkt {
            msg_type: 0u8,
            send_time: 0f64,
            expiry: 0u8,
            send_count: 0,
            data: [0u8;1024],
        }
    }

    pub fn copy_data(&mut self, org: [u8;1024] ) {
        self.data[0..].copy_from_slice(&org[0..]);
    }

    pub fn put_que (& self, queue: &mut MsgQue<EncapPkt>) {
        queue.push(self.clone());
    }

}

pub struct MsgQue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl <T> MsgQue <T> {
    pub fn new() -> Self {
        MsgQue {
            queue: Arc::new(Mutex::new( VecDeque::new())),
        }
    }

    pub fn push(&mut self, value: T) {
        self.queue.lock().unwrap().push_back(value);
    }

    pub fn pop(&mut self) -> T {
        self.queue.lock().unwrap().pop_front().unwrap()
    }

    pub fn check_timer (&self ) {
        let mut expired = vec![]; //for expired timer
        loop {
            tokio::time::sleep(Duration::from_millis(100 as u64)); //sleep 100ms

            let now = tokio::time::Instant::now();

            let queue = self.queue.lock().unwrap();
            queue.iter().for_each(|x: &EncapPkt|{
                if (now-x.send_time > x.expiry) {
                    expired.push(x);
                }
            });

            if expired.len() > 0  {
                for item in expired {
                    if item.send_count < 3 {
                        println!("Resend ");
                        item.send_time = now;
                        item.send_count += 1;
                        if item.msg_type == GTPV2C_ECHO_REQ {
                            item.expiry = ECHO_TIMEOUT;
                        }
                        

                    }
                    else {
                        expired.pop(item);
                    }
                }
            }
        }
    }
}