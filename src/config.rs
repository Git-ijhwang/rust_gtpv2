
use std::fs::File;
use std::io::Result;
use std::collections::HashMap;
use std::str::FromStr;
// use crate::config::io::BufReader;
use crate::gtp_msg::Peer;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use std::sync::{Arc, RwLock};
use std::net::Ipv4Addr;
use crate::gtp_msg::*;

#[derive(Debug)]
pub struct ConfigMap {
    data: HashMap<String, String>,
}

lazy_static::lazy_static! {
    pub static ref CONFIG_MAP: Arc<RwLock<ConfigMap>> = Arc::new(RwLock::new(ConfigMap::new()));
}


impl ConfigMap
{
    pub fn new() -> Self {
        Self{ 
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, target: &str) -> Option<String> {
        self.data.get(target).cloned()
    }

    pub fn create_src_bind_addr(&self) -> String {
        let mut result = String::new();
        let addr = ConfigMap::get(self, "Addr");
        let port  = ConfigMap::get(self, "SrcPort");
        println!("{:?},{:?}", addr, port);

        if addr.is_some() && port.is_some() {
            result = format!("{}:{}", addr.unwrap(), port.unwrap());
            return result;
        }
        return result;
    }
}



pub fn read_conf(file: &str , config: &mut ConfigMap) -> Result<()>
{
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut configline = line?;

        //Skip the comment line with '#'
        let position = configline.trim().find('#');
        if position == Some(0) {
            continue;
        }

        //Remove string after '#'
        if let Some(pos) = configline.find('#') {
            configline = configline[..pos].trim().to_string();
        }

        //Key, Value pair
        if let Some(pos) = configline.find('=') {
            let key = configline[..pos].trim().to_string();
            let value = configline[pos+1..].trim().to_string();

            // let mut config = CONFIG_MAP.write().unwrap();
            config.insert(key, value);
        }
    }

    Ok(())

}

pub fn read_peer(file: &str ) -> Result<()>
{
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut configline = line?;

        //Skip the comment line with '#'
        let position = configline.trim().find('#');
        if position == Some(0) {
            continue;
        }

        //Remove string after '#'
        if let Some(pos) = configline.find('#') {
            configline = configline[..pos].trim().to_string();
        }

        //Key, Value pair
        if let Some(pos) = configline.find('=') {
            let svr = configline[..pos].trim().to_string();
            let conn = configline[pos+1..].trim().to_string();
            let peer;

            let parts: Vec<&str> = conn.split(':').collect();

            let ip = Ipv4Addr::from_str(parts[0]).unwrap();
            let port = u16::from_str(parts[1]).unwrap();
            peer = Peer::new(ip, port);

            let mut peerlist = GTP2_PEER.lock().unwrap();
            peerlist.insert(u32::from(peer.ip), peer);
        }
    }

    Ok(())

}


