use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::config::*;


lazy_static::lazy_static! {
    pub static ref IPPOOL: Arc<Mutex<VecDeque<Ipv4Addr>>> = Arc::new(Mutex::new(VecDeque::new()));
}


pub fn add_ip(ip: Ipv4Addr) {
    let mut pool_lock = IPPOOL.lock().unwrap();
    pool_lock.push_back(ip);
}


pub fn allocate_ip() -> Ipv4Addr {
    let mut pool_lock = IPPOOL.lock().unwrap();
    let ip = pool_lock.pop_front();
    return ip.unwrap();
}


#[allow(dead_code)]
pub fn print_ippool() {
    let pool_lock = IPPOOL.lock().unwrap();
    println!("IPPOOL: {:#?}", pool_lock);
}


pub fn prepare_ip_pool() {
    let config = CONFIG_MAP.read().unwrap();

    let prefix = config.get("PREFIX_V4").unwrap();
    let prefix_addr = Ipv4Addr::from_str(&prefix).unwrap();

    let netmask = config.get("NETMASK_V4").unwrap();
    let netmask_addr = Ipv4Addr::from_str(&netmask).unwrap();

    let prefix_int = u32::from(prefix_addr);
    let netmask_int = u32::from(netmask_addr);
    let network_start = prefix_int & netmask_int;
    let broadcast_addr = network_start | !netmask_int;

    for ip in (network_start+1)..broadcast_addr {
        add_ip(Ipv4Addr::from(ip));
    }
}


