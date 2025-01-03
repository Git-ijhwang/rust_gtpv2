use rand::Rng;
use dashmap::DashMap;
use std::net::Ipv4Addr;
use std::sync::{Arc, RwLock, Mutex};
use crate::gtpv2_recv::{BearerQos, BearerCnxt};
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone)]
pub struct TeidList {
    // teid_map: DashMap<u32, Arc<Mutex<String>>>,
    teid_map: DashMap<u32, Arc<Mutex<Session>>>,
}

impl TeidList {
    pub fn new() -> Self {
        TeidList {
            teid_map: DashMap::new()
        }
    }
    // pub fn find_session_by_teid(&self, teid: &u32) -> Result<String, String> {
    //     // let ret = 
    //     match self.teid_map.get(teid)//.map(|entry| Arc::clone(entry.value()))
    //     {
    //         Some(value) => return Ok(value.clone()),
    //         _ => return Err("Can't fine IMSI by teid".to_string()),
    //     }
    // }

	pub fn add(&self, teid: u32, session: Arc<Mutex<Session>>) {
		self.teid_map.insert(teid, session);
	}

    pub fn del(&self, teid: &u32) {
        self.teid_map.remove(teid);
    }

    pub fn get (&self, teid:&u32) -> Option<Arc<Mutex<Session>>> {
        self.teid_map.get(teid).map(|entry| Arc::clone(&entry))
    }
}


#[derive(Debug, Clone)]
pub struct SessionList {
    // sess_map: DashMap<String, Arc<Mutex<Session>>>,
    sess_map: DashMap<String, Arc<Mutex<Session>>>,
}


impl SessionList {
	pub fn new() -> Self {
		SessionList {
			sess_map: DashMap::new()
		}
	}

    fn find_session_by_imsi(&self, imsi: String)
        -> Result<Arc<Mutex<Session>>, String> {
        match self.sess_map.get(&imsi) {
            Some(v) => Ok(v.clone()),
            _ => Err ("The session is not exist".to_string()),
        }
    }

    pub fn create_session(&self, imsi:String) -> Arc<Mutex<Session>> {
        let mut session = Session::new();
        session.imsi = imsi.clone();

        Arc::new( Mutex::new(session))
    }

	pub fn add (&self, imsi: String, session: Arc<Mutex<Session>>) {
		self.sess_map.insert(imsi, session);
	}

	pub fn del(&self, imsi: &str) {
		self.sess_map.remove(imsi);
	}

	pub fn get (&self, imsi:&str) -> Option<Arc<Mutex<Session>>> {
		self.sess_map.get(imsi).map(|entry| Arc::clone(&entry))
	}

}


#[derive(Debug, Clone)]
pub struct GtpIfaceType {
    pub teid:       u32,
    pub iface_type: u8,
    pub peerip: Ipv4Addr,
}
impl GtpIfaceType {
    fn new() -> Self {
        GtpIfaceType {
            teid:       0,
            iface_type: 0,
            peerip:     Ipv4Addr::new(0,0,0,0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct bearer_info {
    pub used:       bool,
    pub ebi:        u8,
    pub lbi:        u8,
    pub ifaces:     Vec<GtpIfaceType>,
    // pub instance:   u8,
    // pub reserved:   u8,
    // pub pci:        u8,
    // pub pl:         u8,
    // pub/Tei pvi:        u8,
	pub QoS:		BearerQos,
    // s5_u: teid_info_t /* SGW <-> PGW (GTP-U) */
}
impl bearer_info {
    pub fn new() -> Self {
        bearer_info {
            used:   true,
            ebi:    0,
            lbi:    0,
            ifaces: Vec::new(),
			QoS:	BearerQos::new(),
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
    pub teid:       u32,
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
			teid:       0,
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

    pub fn bearer_cnt_check(&self) -> bool {
        if self.bearer.len() >= 11 {
            return false;
        }
        return true;
    }
}


pub fn generate_teid() -> Option<u32> {
    let mut rng = rand::thread_rng();
    Some(rng.gen()) // u32 랜덤 값 생성
}


pub fn find_session_or_create(imsi: String, sess_list: &SessionList) 
-> Result<Arc<Mutex<Session>>, String>
{
    let locked_session = sess_list.find_session_by_imsi(imsi.clone());

    match locked_session {
        Ok(value) => 
            return Err(format!("Fail to find session by IMSI[{}]",imsi).to_string()),
        _ => {
            warn!("Fail to find session by IMSI: {} and create", imsi);
				let sess = sess_list.create_session(imsi.clone());
				sess_list.add(imsi.clone(), sess.clone());
            return Ok (sess)
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

pub fn alloc_pdn( session: &mut Arc<Mutex<Session>>,
	lbi: u8, alloc_ip: Ipv4Addr,
	ambr_dl: u32, ambr_ul: u32, apn:String)
-> Result<usize,String>
{
	let mut session = session.lock().unwrap();
    let pdn_index = session.pdn.len();
    if pdn_index >= 3 {
        return Err("PDN is full".to_string());
    }

    let mut pdn : pdn_info = pdn_info::new();
	pdn.lbi		= lbi;
    pdn.ip		= alloc_ip;
    pdn.ambr_dl = ambr_dl;
    pdn.ambr_ul = ambr_ul;
    pdn.apn		= apn;

	session.pdn.push(pdn);
	return Ok(session.pdn.len());
}

pub fn modify_bearer(bearer: &mut bearer_info, bearer_info: BearerCnxt)
{
    for iface_info in bearer_info.iface_info {
        if iface_info.used == false {
            continue;
        }

        if bearer.ebi != bearer_info.ebi {
            continue;
        }

        let mut iface = GtpIfaceType::new();
        iface.iface_type =	iface_info.iface_type;
        iface.peerip =	    iface_info.addr;
        iface.teid =	    iface_info.teid;

        bearer.ifaces.push(iface);
    }
}

pub fn alloc_bearer(session: &mut Arc<Mutex<Session>>,
    lbi: u8, bearers: Vec<BearerCnxt>)
-> Result<usize,String>
{
	let mut session = session.lock().unwrap();

    if session.bearer_cnt_check() == false {
        return Err("Bearer is full".to_string());
    }

    for bearer_info in bearers {

        let mut bearer = bearer_info::new();
        bearer.ebi=		bearer_info.ebi;
        bearer.lbi=		lbi;

        for iface_info in bearer_info.iface_info {
            if iface_info.used == false {
                continue;
            }

            let mut iface = GtpIfaceType::new();
            iface.teid =	    iface_info.teid;
            iface.peerip =	    iface_info.addr;
            iface.iface_type =	iface_info.iface_type;

            bearer.ifaces.push(iface);
        }
        bearer.QoS =	bearer_info.bearer_qos;
        session.bearer.push(bearer);
    }

    Ok(session.bearer.len())
}


pub fn find_bearer(session: Arc<Mutex<Session>>, ebi: u8)
-> Option<usize>
{
    let mut session = session.lock().unwrap();

    // let test =
    session.bearer.iter()
        .position(|bearer| bearer.ebi == ebi)

    // return test;
}


pub fn delete_pdn_and_bearer(session: &mut Arc<Mutex<Session>>, ebi: u8) {
	let mut session = session.lock().unwrap();
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