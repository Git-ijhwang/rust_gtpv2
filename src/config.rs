use std::fs::File;
use std::io;
use std::io::Error;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// use std::sync::RwLockWriteGuard;
use core::result::Result;


lazy_static::lazy_static! {
    pub static ref CONFIG_MAP: Arc<RwLock<ConfigMap>> = Arc::new(RwLock::new(ConfigMap::new()));
}


lazy_static::lazy_static! {
    pub static ref PEER_MAP: Arc<RwLock<ConfigMap>> = Arc::new(RwLock::new(ConfigMap::new()));
}


#[derive(Debug)]
pub struct ConfigMap {
    data: HashMap<String, String>,
}


impl ConfigMap
{
    pub fn new() -> Self {
        Self { 
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, target: &str) -> Option<String> {
        self.data.get(target).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }

    #[allow(dead_code)]
    pub fn create_src_bind_addr(&self) -> String {
        let mut result = String::new();
        let addr = ConfigMap::get(self, "Addr");
        let port  = ConfigMap::get(self, "SrcPort");
        // println!("{:?},{:?}", addr, port);

        if addr.is_some() && port.is_some() {
            result = format!("{}:{}", addr.unwrap(), port.unwrap());
            return result;
        }
        return result;
    }
}


pub async fn read_conf (path: &str, peer: bool)
        -> Result<(), Error>
{
    // let mut config;
    // let mut config: Option<RwLockWriteGuard<'_, ConfigMap>> = None;
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut config = if peer {
        PEER_MAP.write().await
    }
    else {
        CONFIG_MAP.write().await
    };

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

            config.insert(key, value);
        }
    }
    Ok(())
}
