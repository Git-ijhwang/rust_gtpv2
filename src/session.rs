use rand::Rng;
use std::hash::Hash;
use dashmap::DashMap;
use std::net::Ipv4Addr;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};

// lazy_static::lazy_static! {
//     pub static ref TEID_LIST: Arc<Mutex<TeidList>> =
//         Arc::new(Mutex::new(TeidList::new()));
// }

// lazy_static::lazy_static! {
//     pub static ref SESSION: Arc<RwLock<SessionList>> =
//         Arc::new( RwLock::new( SessionList::new()));
// }


#[derive(Debug, Clone)]
pub struct TeidList {
    teid_map: DashMap<u32, Arc<Mutex<String>>>,
}


#[derive(Debug, Clone)]
pub struct SessionList {
    sess_map: DashMap<String, Arc<Mutex<Session>>>,
}


impl TeidList {
    pub fn new() -> Self {
        TeidList {
            teid_map: DashMap::new()
        }
    }

    pub fn find_session_by_teid(&self, teid: &u32) -> Option<Arc<Mutex<String>>> {
        self.teid_map.get(teid).map(|entry| Arc::clone(entry.value()))
    }

    pub fn add_teid(&self, teid: u32, imsi: &str) {
        self.teid_map.insert(teid, Arc::new(Mutex::new(imsi.to_string())));
    }

    pub fn del_teid(&self, teid: &u32) {
        self.teid_map.remove(teid);
    }
}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
}

impl SessionList {
    pub fn new() -> Self {
        SessionList {
            sess_map: DashMap::new()
        }
    }

    pub fn find_session_by_imsi(&self, imsi: &str) -> Option<Arc<Mutex<Session>>> {
        self.sess_map.get(imsi).map(|entry| Arc::clone(entry.value()))
    }

    /*
    // pub fn mut_session_by_imsi(&self, imsi: &str) -> Option<Arc<Session>> {
    //     self.sess_map.get_mut(imsi)
    // }
    */

    pub fn create_session(&self, imsi:String) {
        let session = Arc::new(Mutex::new(Session::new()));
        session.lock().unwrap().imsi = imsi.clone();
        self.sess_map.insert(imsi,session);
    }

    pub fn del_session(&self, imsi: &str) {
        self.sess_map.remove(imsi);
    }
}




pub fn generate_teid() -> Option<u32> {
    let mut rng = rand::thread_rng();
    Some(rng.gen()) // u32 랜덤 값 생성
}

