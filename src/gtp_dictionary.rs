use std::fs::File;
use std::sync::{Arc, RwLock};
use std::io::{BufReader, Error};
use core::result::Result;
use serde::{Deserialize, Serialize};
// use std::sync::RwLockWriteGuard;
// use std::path::Path;
// use std::fs::File;
// use std::io;
// use std::io::Error;

// GTP Message Structure
#[derive(Debug, Serialize, Deserialize)]
pub struct IeInfo {
    pub ie_type: u8,
    pub min_length: usize,
    pub max_length: usize,
    pub reserved: u8,
    pub instance: u8,
    pub presence: String,
    pub group_ie_info: Option<Vec<IeInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GtpMessage {
    pub msg_type: u8,
    pub msg_name: String,
    pub ie_counts: usize,
    pub ie_list: Vec<IeInfo>,
}


impl GtpMessage {
    pub fn new() -> Self {
        Self {
            msg_type:0,
            msg_name: String::new(),
            ie_counts: 0,
            ie_list: Vec::new(),
        }
    }

    pub fn get_ie_list (&self) -> &Vec<IeInfo> {
        &self.ie_list
    }
}

lazy_static::lazy_static! {
    pub static ref GTP_DICTIONARY: Arc<RwLock<Vec<GtpMessage>>> =
        Arc::new(RwLock::new(
            // Vec::new(
                // <GtpMessage::new()>
            // )
            vec![]
        ));
}


pub fn load_gtp_dictionary(file_path: &str)
// -> Vec<GtpMessage>
-> Result <(), Error>
{
    let file = File::open(file_path)?;
    // .expect("Failed to open JSON file");
    let reader = BufReader::new(file);

    let message: GtpMessage = serde_json::from_reader(reader)
        .map_err( |e| {
            eprintln!("Failed to parse JSON: {}", e);
            e
        })?;

    println!("{:#?}", message);

    if let Ok(mut dictionary) = GTP_DICTIONARY.write() {
        dictionary.push(message);
        Ok(())
    }
    else {
        Err(Error::new (
            std::io::ErrorKind::Other,
            "Failed to acquire write lock",
        ))
    }

    // GTP_DICTIONARY.write().unwrap().push(
    //     serde_json::from_reader(reader).expect("Failed to parse JSON")
    // );
}
