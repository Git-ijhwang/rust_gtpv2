use rand::Rng;
use std::net::Ipv4Addr;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};

lazy_static::lazy_static! {
    pub static ref TEID_LIST: Arc<RwLock<TeidList>> = Arc::new(RwLock::new(TeidList::new()));
}

pub fn generate_teid() -> Option<u32> {
    let mut rng = rand::thread_rng();
    Some(rng.gen()) // u32 랜덤 값 생성
}

pub struct TeidList {
    teid: Vec<u32>,
}

impl TeidList {
    pub fn new() -> Self {
        TeidList {
            teid: vec![0u32; 1024],
        }
    }

    pub fn find_teid(&self, teid: u32) -> bool {
        for id in &self.teid {
            if teid == *id {
                return true;
            }
        }
        return false;
    }

    pub fn put_teid(& mut self, teid: u32) {
        if !self.find_teid(teid) {
            &self.teid.push(teid);
        }
    }

}


lazy_static::lazy_static! {
    pub static ref SESSION: Mutex<HashMap<String, Session>> = Mutex::new(HashMap::new());
}

pub struct bearer_info {
    pub used:       u8,
    pub ebi:        u8,
    pub instance:   u8,
    pub reserved:   u8,
    pub pci:        u8,
    pub pl:         u8,
    pub pvi:        u8,
    pub qci:        u8,
    pub mbr_up:     u32,
    pub mbr_down:   u32,
    pub gbr_up:     u32,
    pub gbr_down:   u32,
    pub teid:       u32,
    // s1_u: teid_info_t /* MME <-> SGW (GTP-U) */
    // s5_u: teid_info_t /* SGW <-> PGW (GTP-U) */
}

pub struct Session {
    pub imsi:       String,
	// teid_list_t			s11;		/* MME <-> SGW (GTP-C) */
	// teid_list_t			s5;			/* SGW <-> PGW (GTP-C) */

    pub teid:       u32,
    pub bearer:     Vec<bearer_info>,

    pub peertype:   u8,
    pub status:     u8,
    pub seqnum:     u32,

    pub ip:         Ipv4Addr,
    pub ambr_up:    u32,
    pub ambr_down:  u32,
}

impl Session {
    pub fn new() -> Self {
        Session {
            imsi:       String::new(),
            teid:       0,
            bearer:     Vec::with_capacity(4),
            peertype:   0,
            status:     0, 
            seqnum:     0, 
            ip:         Ipv4Addr::new(0,0,0,0),
            ambr_up:    0,
            ambr_down:  0,
        }
    }

    pub fn find_session(imsi: String) {
        let session = SESSION.lock().unwrap();

    }
}