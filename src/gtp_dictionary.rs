use std::fs::File;
use std::sync::Arc;
use tokio::sync::RwLock;
use core::result::Result;
use std::io::{BufReader, Error };
use serde::{Deserialize, Serialize};
use log::{debug, error, info, trace, warn};

// GTP Message Structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IeInfo {
    pub ie_type:    u8,
    pub min_length: usize,
    pub max_length: usize,
    pub reserved:   u8,
    pub instance:   u8,
    pub presence:   String,
    pub group_ie_info: Option<Vec<IeInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GtpMessage {
    pub msg_type:   u8,
    pub msg_name:   String,
    pub response:   Vec<u32>,
    pub ie_counts:  usize,
    pub ie_list:    Vec<IeInfo>,
}


impl GtpMessage {

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            msg_type:   0,
            msg_name:   String::new(),
            response:   Vec::new(),
            ie_counts:  0,
            ie_list:    Vec::new(),
        }
    }
}


lazy_static::lazy_static! {
    pub static ref GTP_DICTIONARY: Arc<RwLock<Vec<GtpMessage>>> =
        Arc::new(RwLock::new(vec![]));
}


pub async fn print_dictionary () {
    info!("{:#?}",GTP_DICTIONARY.read().await.clone());
}


pub async fn load_gtp_dictionary(file_path: &str)
-> Result <(), Error>
{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let messages: Vec<GtpMessage> = serde_json::from_reader(reader)
        .map_err( |e| {
            error!("Failed to parse the JSON file: {}", e);
            eprintln!("Failed to parse JSON: {}", e);
            e
        })?;

    let mut dictionary = GTP_DICTIONARY.write().await;
    for message in messages {
            dictionary.push(message);
    }

    info!("{:#?}", dictionary);
    Ok(())
}
