use rand::Rng;
use dashmap::DashMap;
use std::net::Ipv4Addr;
use std::sync::{Arc, RwLock, Mutex};
use log::{debug, error, info, trace, warn};

lazy_static::lazy_static! {
    pub static ref TEID_LIST: Arc<Mutex<TeidList>> =
        Arc::new(Mutex::new(TeidList::new()));
}

lazy_static::lazy_static! {
    pub static ref SESSION_LIST: Arc<Mutex<SessionList>> =
        Arc::new( Mutex::new( SessionList::new()));
}


#[derive(Debug, Clone)]
pub struct TeidList {
    // teid_map: DashMap<u32, Arc<Mutex<String>>>,
    teid_map: DashMap<u32, String>,
}


#[derive(Debug, Clone)]
pub struct SessionList {
    // sess_map: DashMap<String, Arc<Mutex<Session>>>,
    sess_map: DashMap<String, Session>,
}


impl SessionList {
    fn find_session_by_imsi(&self, imsi: String)
        -> Result<Session, String> {
        match self.sess_map.get(&imsi) {
            Some(v) => Ok(v.clone()),
            _ => Err ("The session is not exist".to_string()),
        }
    }

}


impl TeidList {
    pub fn new() -> Self {
        TeidList {
            teid_map: DashMap::new()
        }
    }

    pub fn find_session_by_teid(&self, teid: &u32) -> Result<String, String> {
        // let ret = 
        match self.teid_map.get(teid)//.map(|entry| Arc::clone(entry.value()))
        {
            Some(value) => return Ok(value.clone()),
            _ => return Err("Can't fine IMSI by teid".to_string()),
        }
    }

    pub fn add_teid(&self, teid: u32, imsi: &str) {
        self.teid_map.insert(teid, imsi.to_string());
    }

    pub fn del_teid(&self, teid: &u32) {
        self.teid_map.remove(teid);
    }
}


#[derive(Debug, Clone)]
pub struct bearer_info {
    pub used:       bool,
    pub ebi:        u8,
    pub lbi:        u8,
    pub teid:       u32,
    // pub peerip:     Ipv4Addr,
    // pub instance:   u8,
    // pub reserved:   u8,
    // pub pci:        u8,
    // pub pl:         u8,
    // pub/Tei pvi:        u8,
    pub flag:       u8,
    pub qci:        u8,
    pub mbr_ul:     u32,
    pub mbr_dl:     u32,
    pub gbr_ul:     u32,
    pub gbr_dl:     u32,
    // s5_u: teid_info_t /* SGW <-> PGW (GTP-U) */
}
impl bearer_info {
    pub fn new() -> Self {
        bearer_info {
            used:   true,
            ebi:    0,
            lbi:    0,
            teid:   0,
            // peerip: Ipv4Addr::new(0,0,0,0),
            flag:   0,
            qci:    0,
            mbr_ul: 0,
            mbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
        }
    }
}


#[derive(Debug, Clone)]
pub struct pdn_info {
    pub used:        bool,
    pub lbi:         u8,
    pub ip:          Ipv4Addr, //allocated IP Address for UE
    pub apn:         String,
    pub ambr_ul:     u32,
    pub ambr_dl:     u32,
}

impl pdn_info {
    pub fn new() -> Self {
        pdn_info {
            used :   true,
            lbi :    0,
            ip :     Ipv4Addr::new(0,0,0,0),
            apn:     String::new(),
            ambr_ul: 0,
            ambr_dl: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct control_info {
    pub interface_type: u8,
    pub teid:           u32,
    pub addr:           Ipv4Addr,
}

impl control_info {
    pub fn new() -> Self {
        control_info {
            interface_type: 0,
            teid:           0,
            addr:           Ipv4Addr::new(0,0,0,0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub imsi:       String,
    pub msisdn:     String,
	// teid_list_t			s11;		/* MME <-> SGW (GTP-C) */
	// teid_list_t			s5;			/* SGW <-> PGW (GTP-C) */

    pub control:    Vec<control_info>,
    pub bearer:     Vec<bearer_info>,
    pub pdn:        Vec<pdn_info>,

    // pub peerip:     Ipv4Addr,
    pub status:     u8,     //Session Status
    pub seqnum:     u32,

    // pub ip:         Ipv4Addr,
    // pub peer:       Peer,
}


impl Session {
    pub fn new() -> Self {
        Session {
            imsi:       String::new(),
            msisdn:     String::new(),
            control:    Vec::with_capacity(4),
            bearer:     Vec::with_capacity(11),
            pdn:        Vec::with_capacity(3),
            // peertype:   0,
            status:     0, 
            seqnum:     0, 
            // ip:         Ipv4Addr::new(0,0,0,0),
            // peer:       Peer::new(
                // Ipv4Addr::new(0,0,0,0), 0, "".to_string()),
        }
    }
}

impl SessionList {
    pub fn new() -> Self {
        SessionList {
            sess_map: DashMap::new()
        }
    }


    pub fn create_session(&self, imsi:String) -> Session {
        let mut session = Session::new();
        session.imsi = imsi.clone();

        session
    }
}


pub fn generate_teid() -> Option<u32> {
    let mut rng = rand::thread_rng();
    Some(rng.gen()) // u32 랜덤 값 생성
}


pub fn get_imsi_by_teid(teid: u32) -> Result<String, String>
{
    trace!("Find Session by TEID");
    let teid_list = TEID_LIST.lock().unwrap();
    match teid_list.find_session_by_teid(&teid) {
        Ok(value) => return Ok(value.clone()),
        _ => {
            warn!("Fail to find session by TEID: {}", teid);
            return Err("Error".to_string())
        }
    }
}


pub fn find_session_by_imsi(imsi: String) 
-> Result<Session, String>
{
    let sessionlist = SESSION_LIST.lock().unwrap();
    let locked_session = sessionlist.find_session_by_imsi(imsi.clone());

    match locked_session {
        Ok(value) => Ok(value),
        _ => {
            warn!("Fail to find session by IMSI: {}", imsi);
            return Err("Fail to find session by IMSI".to_string())
        }

    }
}


pub fn find_session_or_create(imsi: String) 
-> Result<Session, String>
{
    let sessionlist = SESSION_LIST.lock().unwrap();
    let locked_session = sessionlist.find_session_by_imsi(imsi.clone());
    // let arc_session;

    match locked_session {
        Ok(value) => 
            return Err(format!("Fail to find session by IMSI[{}]",imsi).to_string()),
        _ => {
            warn!("Fail to find session by IMSI: {} and create", imsi);
            return Ok (sessionlist.create_session(imsi.clone()));
        }
    }
}


pub fn check_pdn(session: &Session, lbi: u8) -> bool {
    session.pdn.iter()
    .any(|pdn| pdn.lbi == lbi)
}


pub fn check_bearer_by_lbi(session: &Session, lbi: u8) -> bool {
    session.bearer.iter()
    .any(|bearer| bearer.lbi == lbi)
}


pub fn check_bearer_by_ebi(session: &Session, ebi: u8) -> bool {
    session.bearer.iter()
    .any(|bearer| bearer.ebi == ebi)
}


pub fn find_empty_pdn(session: &mut Session)
-> Option<&mut pdn_info>
{
    session.pdn.iter_mut()
    .find(|pdn|pdn.used == false)
}

pub fn find_pdn(session: &mut Session, ebi: u8)
-> Option<&mut pdn_info>
{
    session.pdn.iter_mut()
    .find(|pdn|pdn.lbi == ebi)
}

pub fn alloc_pdn(session: &mut Session, lbi: u8, alloc_ip: Ipv4Addr, ambr_dl: u32, ambr_ul: u32, apn:String)
-> Result<usize,String>
{
    let pdn_index = session.pdn.len();
    if pdn_index >= 3 {
        return Err("PDN is full".to_string());
    }

    let mut pdn : pdn_info = pdn_info::new();
    pdn.ip      = alloc_ip;
    pdn.ambr_dl = ambr_dl;
    pdn.ambr_ul = ambr_ul;
    pdn.apn     = apn;

    session.pdn.push(pdn);

    Ok(session.pdn.len())
}


pub fn alloc_bearer(session: &mut Session,
    lbi: u8, ebi: u8, teid: u32, flag: u8, qci: u8,
    mbr_ul: u32, mbr_dl: u32, gbr_ul: u32, gbr_dl: u32)
-> Result<usize,String>
{
    let bearer_index = session.bearer.len();
    if bearer_index >= 11 {
        return Err("Bearer is full".to_string());
    }

    let mut bearer = bearer_info::new();
    bearer.ebi=    ebi;
    bearer.lbi=    lbi;
    bearer.teid=   teid;
    bearer.flag=   flag;
    bearer.qci=    qci;
    bearer.mbr_ul= mbr_ul;
    bearer.mbr_dl= mbr_dl;
    bearer.gbr_ul= gbr_ul;
    bearer.gbr_dl= gbr_dl;

    session.bearer.push(bearer);
    Ok(session.bearer.len())
}


pub fn find_bearer(session: &mut Session, ebi: u8)
-> Vec<&mut bearer_info>
{
    session.bearer
    .iter_mut()
    .find(|bearer|bearer.lbi == ebi)
    .into_iter()
    .collect()
}


pub fn delete_pdn_and_bearer(session: &mut Session, ebi: u8) {
    if let Some(pdn_index) = session.pdn.iter().position(|pdn| pdn.lbi == ebi) {

        let pdn = session.pdn.remove(pdn_index);
        info!("Deleted pdn: {:?}", pdn);

        // Bearer 삭제
        let bearer_indices: Vec<usize> = session.bearer
            .iter()
            .enumerate()
            .filter(|(_, bearer)| bearer.lbi == ebi)
            .map(|(i, _)| i)
            .collect();

        for index in bearer_indices.into_iter().rev() {
            let bearer = session.bearer.remove(index);
            info!("Deleted bearer: {:?}", bearer);
        }

    } else {
        info!("No pdn found with ebi: {}", ebi);
    }
}