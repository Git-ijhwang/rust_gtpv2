use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::Path;

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
    pub fn  get_ie_list (&self) -> &Vec<IeInfo> {
        &self.ie_list
    }
}

pub fn load_gtp_dictionary(file_path: &str) -> Vec<GtpMessage> {
    let file = File::open(file_path).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse JSON")
}
