/*
 		[ 3GPP TS 29.274 V10.5.0 (2011-12) ]
*/

const GTP_VERSION: i32 = 	                    	            0x02;
const GTPV2_P_FLAG: i32 =                    		            0x10;
const GTPV2_T_FLAG: i32 = 	     	                            0x08;
const GTPV2C_MINIMUM_HEADER_SIZE: i32 = 		                8;
const GTPV2C_EPC_SPECIFIC_HEADER_SIZE: i32 = 		            12;


/*
 * GTPv2-C Message Type Values
 * ======================================================
 */
pub const GTPV2C_ECHO_REQ: u8 = 		                            1;
pub const GTPV2C_ECHO_RSP: u8 = 		                            2;
const GTPV2C_VERSION_NOT_SUPPORTED_IND: u8 = 		            3;
 		/* ----------------------------------------- *
 		 * SGSN/MME/ePDG to PGW (S4/S11, S5/S8, S2b) *
 		 * ----------------------------------------- */
const GTPV2C_CREATE_SESSION_REQ: u8 = 		                    32;
const GTPV2C_CREATE_SESSION_RSP: u8 = 		                    33;
const GTPV2C_DELETE_SESSION_REQ: u8 = 		                    36;
const GTPV2C_DELETE_SESSION_RSP: u8 = 		                    37;
 		/* ------------------------------- *
 		 * SGSN/MME to PGW (S4/S11, S5/S8) *
 		 * ------------------------------- */
const GTPV2C_MODIFY_BEARER_REQ: u8 = 		                    34;
const GTPV2C_MODIFY_BEARER_RSP: u8 = 		                    35;
const GTPV2C_CHANGE_NTF_REQ: u8 = 		                        38;
const GTPV2C_CHANGE_NTF_RSP: u8 = 		                        39;
const GTPV2C_RESUME_NTF: u8 = 		                            164;
const GTPV2C_RESUME_ACK: u8 = 		                            165;
 		/* ---------------------------------- *
 		 * Messages without explicit response *
 		 * ---------------------------------- */
const GTPV2C_MODIFY_BEARER_CMD: u8 = 		                    64; 		/* MME/SGSN/ePDG to PGW - S11/S4, S5/S8, S2b */
const GTPV2C_MODIFY_BEARER_FAILURE_IND: u8 = 		            65; 		/* PGW to MME/SGSN/ePDG - S5/S8, S11/S4, S2b */
const GTPV2C_DELETE_BEARER_CMD: u8 = 		                    66; 		/* MME/SGSN to PGW - S11/S4, S5/S8 */
const GTPV2C_DELETE_BEARER_FAILURE_IND: u8 = 		            67; 		/* PGW to MME/SGSN -S5/S8, S11/S4 */
const GTPV2C_BEARER_RESOURCE_CMD: u8 = 		                68; 		/* MME/SGSN to PGW - S11/S4, S5/S8 */
const GTPV2C_BEARER_RESOURCE_FAILURE_IND: u8 = 	            69; 		/* PGW to MME/SGSN -S5/S8, S11/S4 */
const GTPV2C_DOWNLINK_DATA_NOTIFICATION_FAILURE_IND: u8 = 		70; 		/* SGSN/MME to SGW - S4/S11 */
const GTPV2C_TRACE_SESSION_ACTIVATION: u8 = 		            71; 		/* MME/SGSN/ePDG to PGW - S11/S4, S5/S8, S2b */
const GTPV2C_TRACE_SESSION_DEACTIVATION: u8 = 		            72; 		/* MME/SGSN/ePDG to PGW - S11/S4, S5/S8, S2b */
const GTPV2C_STOP_PAGING_IND: u8 = 		                    73; 		/* SGW to MME/SGSN - S11/S4 */
 		/* ----------------------------------------- *
 		 * PGW to SGSN/MME/ePDG (S5/S8, S4/S11, S2b) *
 		 * ----------------------------------------- */
const GTPV2C_CREATE_BEARER_REQ: u8 = 		                    95;
const GTPV2C_CREATE_BEARER_RSP: u8 = 		                    96;
const GTPV2C_UPDATE_BEARER_REQ: u8 = 		                    97;
const GTPV2C_UPDATE_BEARER_RSP: u8 = 		                    98;
const GTPV2C_DELETE_BEARER_REQ: u8 = 		                    99;
const GTPV2C_DELETE_BEARER_RSP: u8 = 		                    100;
 		/* ------------------------------------------------------------------------------------------ *
 		 * PGW to MME, MME to PGW, SGW to PGW, SGW to MME, PGW to ePDG, ePDG to PGW (S5/S8, S11, S2b) *
 		 * ------------------------------------------------------------------------------------------ */
const GTPV2C_DELETE_PDN_CONNECTION_SET_REQ: u8 = 		        101;
const GTPV2C_DELETE_PDN_CONNECTION_SET_RSP: u8 = 		        102;
 		/* --------------------------------------------------------------- *
 		 * MME to MME, SGSN to MME, MME to SGSN, SGSN to SGSN (S3/S10/S16) *
 		 * --------------------------------------------------------------- */
const GTPV2C_IDENTIFICATION_REQ: u8 = 		                    128;
const GTPV2C_IDENTIFICATION_RSP: u8 = 		                    129;
const GTPV2C_CONTEXT_REQ: u8 = 		                        130;
const GTPV2C_CONTEXT_RSP: u8 = 		                        131;
const GTPV2C_CONTEXT_ACK: u8 = 		                        132;
const GTPV2C_FORWARD_RELOCATION_REQ: u8 = 		                133;
const GTPV2C_FORWARD_RELOCATION_RSP: u8 = 		                134;
const GTPV2C_FORWARD_RELOCATION_COMPLETE_NTF: u8 = 		    135;
const GTPV2C_FORWARD_RELOCATION_COMPLETE_ACK: u8 = 		    136;
const GTPV2C_FORWARD_ACCESS_CONTEXT_NTF: u8 = 		            137;
const GTPV2C_FORWARD_ACCESS_CONTEXT_ACK: u8 = 		            138;
const GTPV2C_RELOCATION_CANCEL_REQ: u8 = 		                139;
const GTPV2C_RELOCATION_CANCEL_RSP: u8 = 		                140;
const GTPV2C_CONFIGURE_TRANSFER_TUNNEL: u8 = 		            141;
const GTPV2C_RAN_INFORMATION_RELAY: u8 = 		                152;
 		/* ----------------------------- *
 		 * SGSN to MME, MME to SGSN (S3) *
 		 * ----------------------------- */
const GTPV2C_DETACH_NTF: u8 = 		                            149;
const GTPV2C_DETACH_ACK: u8 = 		                            150;
const GTPV2C_CS_PAGING_INDICATION: u8 = 		                151;
const GTPV2C_ALERT_MME_NTF: u8 = 		                        153;
const GTPV2C_ALERT_MME_ACK: u8 = 		                        154;
const GTPV2C_UE_ACTIVITY_NTF: u8 = 		                    155;
const GTPV2C_UE_ACTIVITY_ACK: u8 = 		                    156;
 		/* ---------------------------------------- *
 		 * SGSN/MME to SGW, SGSN to MME (S4/S11/S3) *
         * SGSN to SGSN (S16), SGW to PGW (S5/S8)   *
 		 * ---------------------------------------- */
const GTPV2C_SUSPEND_NTF: u8 = 		                        162;
const GTPV2C_SUSPEND_ACK: u8 = 		                        163;
 		/* ------------------------ *
 		 * SGSN/MME to SGW (S4/S11) *
 		 * ------------------------ */
const GTPV2C_CREATE_FORWARDING_TUNNEL_REQ: u8 = 		        160;
const GTPV2C_CREATE_FORWARDING_TUNNEL_RSP: u8 = 		        161;
const GTPV2C_CREATE_INDIRECT_DATA_FORWARDING_TUNNEL_REQ: u8 = 		166;
const GTPV2C_CREATE_INDIRECT_DATA_FORWARDING_TUNNEL_RSP: u8 = 		167;
const GTPV2C_DELETE_INDIRECT_DATA_FORWARDING_TUNNEL_REQ: u8 = 		168;
const GTPV2C_DELETE_INDIRECT_DATA_FORWARDING_TUNNEL_RSP: u8 = 		169;
const GTPV2C_RELEASE_ACCESS_BEARERS_REQ: u8 = 		            170;
const GTPV2C_RELEASE_ACCESS_BEARERS_RSP: u8 = 		            171;
 		/* ------------------------ *
 		 * SGW to SGSN/MME (S4/S11) *
 		 * ------------------------ */
const GTPV2C_DOWNLINK_DATA_NOTIFICATION: u8 = 		            176;
const GTPV2C_DOWNLINK_DATA_NOTIFICATION_ACK: u8 = 		        177;
const GTPV2C_PGW_RESTART_NOTIFICATION: u8 = 		            179;
const GTPV2C_PGW_RESTART_NOTIFICATION_ACK: u8 = 		        180;
 		/* ------------------------------ *
 		 * SGW to PGW, PGW to SGW (S5/S8) *
 		 * ------------------------------ */
const GTPV2C_UPDATE_PDN_CONNECTION_SET_REQ: u8 = 		        200;
const GTPV2C_UPDATE_PDN_CONNECTION_SET_RSP: u8 = 		        201;
 		/* ---------------- *
 		 * MME to SGW (S11) *
 		 * ---------------- */
const GTPV2C_MODIFY_ACCESS_BEARERS_REQ: u8 = 		            211;
const GTPV2C_MODIFY_ACCESS_BEARERS_RSP: u8 = 		            212;
 		/* --------------------------- *
 		 * MBMS GW to MME/SGSN (Sm/Sn) *
 		 * --------------------------- */
const GTPV2C_MBMS_SESSION_START_REQ: u8 = 		                231;
const GTPV2C_MBMS_SESSION_START_RSP: u8 = 		                232;
const GTPV2C_MBMS_SESSION_UPDATE_REQ: u8 = 		            233;
const GTPV2C_MBMS_SESSION_UPDATE_RSP: u8 = 		            234;
const GTPV2C_MBMS_SESSION_STOP_REQ: u8 = 		                235;
const GTPV2C_MBMS_SESSION_STOP_RSP: u8 = 		                236;
const GTPV2C_MSG_MAX: u8 = 		                            255;


/*
 * GTPv2-C Information Element Type Values
 * ======================================================
 */
const GTPV2C_IE_RESERVED: u8 = 		                        0;
const GTPV2C_IE_IMSI: u8 = 		                            1; 		/* International Mobile Subscriber Identity */
const GTPV2C_IE_CAUSE: u8 = 		                            2;
const GTPV2C_IE_RECOVERY: u8 = 		                        3; 		/* Restart Counter */
 		/* Reserved for S101 interface 		4 to 50 */
const GTPV2C_IE_STN_SR: u8 = 		                            51;
 		/* Reserved for Sv interface 		52 to 70 */
const GTPV2C_IE_APN: u8 = 		                                71; 		/* Access Point Name */
const GTPV2C_IE_AMBR: u8 = 		                            72; 		/* Aggregate Maximum Bit Rate */
const GTPV2C_IE_EBI: u8 = 		                                73; 		/* EPS Bearer ID */
const GTPV2C_IE_IP_ADDRESS: u8 = 		                        74;
const GTPV2C_IE_MEI: u8 = 		                                75; 		/* Mobile Equipment Identity */
const GTPV2C_IE_MSISDN: u8 = 		                            76;
const GTPV2C_IE_INDICATION: u8 = 		                        77;
const GTPV2C_IE_PCO: u8 = 		                                78; 		/* Protocol Configuration Options */
const GTPV2C_IE_PAA: u8 = 		                                79; 		/* PDN Address Allocation */
const GTPV2C_IE_BEARER_QOS: u8 = 		                        80; 		/* Bearer Level Quality of Service */
const GTPV2C_IE_FLOW_QOS: u8 = 		                        81; 		/* Flow Quality of Service */
const GTPV2C_IE_RAT_TYPE: u8 = 		                        82;
const GTPV2C_IE_SERVING_NETWORK: u8 = 		                    83;
const GTPV2C_IE_BEARER_TFT: u8 = 		                        84; 		/* EPS Bearer Level Traffic Flow Template */
const GTPV2C_IE_TAD: u8 = 		                                85; 		/* Traffic Aggregation Description */
const GTPV2C_IE_ULI: u8 = 		                                86; 		/* User Location Information */
const GTPV2C_IE_FTEID: u8 = 		                            87; 		/* Fully Qualified Tunnel Endpoint Identifier */
const GTPV2C_IE_TMSI: u8 = 		                            88;
const GTPV2C_IE_GLOBAL_CN_ID: u8 = 		                    89;
const GTPV2C_IE_S103PDF: u8 = 		                            90; 		/* S103 PDN Data Forwarding Info */
const GTPV2C_IE_S1UDF: u8 = 		                            91; 		/* S1-U Data Forwarding Info */
const GTPV2C_IE_DELAY_VALUE: u8 = 		                        92;
const GTPV2C_IE_BEARER_CONTEXT: u8 = 		                    93;
const GTPV2C_IE_CHARGING_ID: u8 = 		                        94;
const GTPV2C_IE_CHARGING_CHARACTERISTICS: u8 = 		        95;
const GTPV2C_IE_TRACE_INFORMATION: u8 = 		                96;
const GTPV2C_IE_BEARER_FLAGS: u8 = 		                    97;
 		/* Reserved: u8 = 		98 */
const GTPV2C_IE_PDN_TYPE: u8 = 		                        99;
const GTPV2C_IE_PROCEDURE_TRANSACTION_ID: u8 = 		        100;
const GTPV2C_IE_DRX_PARAMETER: u8 = 		                    101;
 		/* Reserved 		102 */
 		/* MM Context 		103 to 108 */
const GTPV2C_IE_PDN_CONNECTION: u8 = 		                    109;
const GTPV2C_IE_PDU_NUMBERS: u8 = 		                        110;
const GTPV2C_IE_PTMSI: u8 = 		                            111;
const GTPV2C_IE_PTMSI_SIGNATURE: u8 = 		                    112;
const GTPV2C_IE_HOP_COUNTER: u8 = 		                        113;
const GTPV2C_IE_UE_TIME_ZONE: u8 = 		                    114;
const GTPV2C_IE_TRACE_REFERENCE: u8 = 		                    115;
const GTPV2C_IE_COMPLETE_REQUEST_MESSAGE: u8 = 		        116;
const GTPV2C_IE_GUTI: u8 = 		                            117;
const GTPV2C_IE_F_CONTAINER: u8 = 		                        118;
const GTPV2C_IE_F_CAUSE: u8 = 		                            119;
const GTPV2C_IE_SELECTED_PLMN_ID: u8 = 		                120;
const GTPV2C_IE_TARGET_IDENTIFICATION: u8 = 		            121;
 		/* Reserved 		122 */
const GTPV2C_IE_PACKET_FLOW_ID: u8 = 		                    123;
const GTPV2C_IE_RAB_CONTEXT: u8 = 		                        124;
const GTPV2C_IE_SOURCE_RNC_PDCP_CONTEXT_INFO: u8 = 		    125;
const GTPV2C_IE_UDP_SOURCE_PORT_NUMBER: u8 = 		            126;
const GTPV2C_IE_APN_RESTRICTION: u8 =                     		127;
const GTPV2C_IE_SELECTION_MODE: u8 = 		                    128;
const GTPV2C_IE_SOURCE_IDENTIFICATION: u8 = 		            129;
 		/* Reserved 		130 */
const GTPV2C_IE_CHANGE_REPORTING_ACTION: u8 = 		            131;
const GTPV2C_IE_FQ_CSID: u8 = 		                            132; 		/* Fully Qualified PDN Connection Set Identifier */
const GTPV2C_IE_CHANNEL_NEEDED: u8 = 		                    133;
const GTPV2C_IE_EMLPP_PRIORITY: u8 = 		                    134;
const GTPV2C_IE_NODE_TYPE: u8 = 		                        135;
const GTPV2C_IE_FQDN: u8 = 		                            136; 		/* Fully Qualified Domain Name */
const GTPV2C_IE_TI: u8 = 		                                137; 		/* Transaction Identifier */
const GTPV2C_IE_MBMS_SESSION_DURATION: u8 = 		            138;
const GTPV2C_IE_MBMS_SERIVCE_AREA: u8 = 		                139;
const GTPV2C_IE_MBMS_SESSION_IDENTIFIER: u8 = 		            140;
const GTPV2C_IE_MBMS_FLOW_IDENTIFIER: u8 = 		            141;
const GTPV2C_IE_MBMS_IP_MULTICAST_DISTRIBUTION: u8 = 		    142;
const GTPV2C_IE_MBMS_DISTRIBUTION_ACK: u8 = 		            143;
const GTPV2C_IE_RFSP_INDEX: u8 = 		                        144;
const GTPV2C_IE_UCI: u8 = 		                                145; 		/* User CSG Information */
const GTPV2C_IE_CSG_INFORMATION_REPORTING_ACTION: u8 = 		146;
const GTPV2C_IE_CSG_ID: u8 = 		                            147;
const GTPV2C_IE_CMI: u8 = 		                                148; 		/* CSG Membership Indication */
const GTPV2C_IE_SERVICE_INDICATOR: u8 = 		                149;
const GTPV2C_IE_DETACH_TYPE: u8 = 		                        150;
const GTPV2C_IE_LDN: u8 = 		                                151; 		/* Local Distiguished Name */
const GTPV2C_IE_NODE_FEATURES: u8 = 		                    152;
const GTPV2C_IE_MBMS_TIME_TO_DATA_TRANSFER: u8 = 		        153;
const GTPV2C_IE_THROTTING: u8 = 		                        154;
const GTPV2C_IE_ARP: u8 = 		                                155; 		/* Allocation/Retention Priority */
const GTPV2C_IE_EPC_TIMER: u8 = 		                        156;
const GTPV2C_IE_SIGNALLING_PRIORITY_INDICATION: u8 =     		157;
const GTPV2C_IE_TMGI: u8 = 		                            158; 		/* Temporary Mobile Group Identity */
const GTPV2C_IE_ADDITIONAL_MM_CONTEXT_FOR_SRVCC: u8 =     		159;
const GTPV2C_IE_ADDITIONAL_FLAGS_FOR_SRVCC: u8 = 		        160;
const GTPV2C_IE_MMBR: u8 = 		                            161; 		/* Max MBR/APN-AMBR */
const GTPV2C_IE_MDT_CONFIGURATION: u8 = 		                162;
const GTPV2C_IE_APCO: u8 = 		                            163; 		/* Additional Protocol Configuration Options */
 		/* Spare. For future use. 		164 to 254 */
const GTPV2C_IE_PRIVATE_EXTENSION: u8 = 		                255;
const GTPV2C_IE_TYPE_MAX: u8 = 		                        255;




pub static gtpv_msg_type_vals: [&'static str; 256] = [
	"Reserved",
	"Echo Request",
	"Echo Response",
	"Version Not Supported Indication",
	"Unknown",		/* - Reserved for S interface TS . */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",	/* - Reserved for Sv interface TS . */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Create Session Request",
	"Create Session Response",
	"Modify Bearer Request",
	"Modify Bearer Response",
	"Delete Session Request",
	"Delete Session Response",
	"Change Notification Request",
	"Change Notification Response",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Modify Bearer Command",
	"Modify Bearer Failure Indication",
	"Delete Bearer Command",
	"Delete Bearer Failure Indication",
	"Bearer Resource Command",
	"Bearer Resource Failure Indication",
	"Downlink Data Notification Failure Indication",
	"Trace Session Activation",
	"Trace Session Deactivation",
	"Stop Paging Indication",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Create Bearer Request",
	"Create Bearer Response",
	"Update Bearer Request",
	"Update Bearer Response",
	"Delete Bearer Request",
	"Delete Bearer Response",
	"Delete PDN Connection Set Request",
	"Delete PDN Connection Set Response",
	"PGW Downlink Triggering Notification",
	"PGW Downlink Triggering Acknowledge",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Identification Request",
	"Identification Response",
	"Context Request",
	"Context Response",
	"Context Acknowledge",
	"Forward Relocation Request",
	"Forward Relocation Response",
	"Forward Relocation Complete Notification",
	"Forward Relocation Complete Acknowledge",
	"Forward Access Context Notification",
	"Forward Access Context Acknowledge",
	"Relocation Cancel Request",
	"Relocation Cancel Response",
	"Configuration Transfer Tunnel",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Detach Notification",
	"Detach Acknowledge",
	"CS Paging Indication",
	"RAN Information Relay",
	"Alert MME Notification",
	"Alert MME Acknowledge",
	"UE Activity Notification",
	"UE Activity Acknowledge",
	"ISR Status Indication",	/* - For future use */
	"UE Registration Query Request",
	"UE Registration Query Response",
	"Create Forwarding Tunnel Request",
	"Create Forwarding Tunnel Response",
	"Suspend Notification",
	"Suspend Acknowledge",
	"Resume Notification",
	"Resume Acknowledge",
	"Create Indirect Data Forwarding Tunnel Request",
	"Create Indirect Data Forwarding Tunnel Response",
	"Delete Indirect Data Forwarding Tunnel Request",
	"Delete Indirect Data Forwarding Tunnel Response",
	"Release Access Bearers Request",
	"Release Access Bearers Response",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Downlink Data Notification",
	"Downlink Data Notification Acknowledgement",
	"Unknown",
	"PGW Restart Notification",
	"PGW Restart Notification Acknowledge",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Update PDN Connection Set Request",
	"Update PDN Connection Set Response",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Modify Access Bearers Request",
	"Modify Access Bearers Response",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"MBMS Session Start Request",
	"MBMS Session Start Response",
	"MBMS Session Update Request",
	"MBMS Session Update Response",
	"MBMS Session Stop Request",
	"MBMS Session Stop Response",	
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",	/* - For future use */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
];
pub enum IEs {
	RESERVED(String),
	IMSI(Vec<u8>),
}

pub static gtpv_ie_type_vals: [&str;256] = [
	"Reserved",
	"International Mobile Subscriber Identity (IMSI)",
	"Cause",
	"Recovery (Restart Counter)",
	"Unknown",		/* - Reserved for S interface */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"STN-SR",
	"Unknown",	/* - Reserved for Sv interface */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Access Point Name (APN)",
	"Aggregate Maximum Bit Rate (AMBR)",
	"EPS Bearer ID (EBI)",
	"IP Address",
	"Mobile Equipment Identity (MEI)",
	"MSISDN",
	"Indication",
	"Protocol Configuration Options (PCO)",
	"PDN Address Allocation (PAA)",
	"Bearer Level Quality of Service (Bearer QoS)",
	"Flow Quality of Service (Flow QoS)",
	"RAT Type",
	"Serving Network",
	"EPS Bearer Level Traffic Flow Template (Bearer TFT)",
	"Traffic Aggregation Description (TAD)",
	"User Location Info (ULI)",
	"Fully Qualified Tunnel Endpoint Identifier (F-TEID)",
	"TMSI",
	"Global CN-Id",
	"S PDN Data Forwarding Info (SPDF)",
	"S-U Data Forwarding Info (SUDF)",
	"Delay Value",
	"Bearer Context",
	"Charging ID",
	"Charging Characteristics",
	"Trace Information",
	"Bearer Flags",
	"Reserved",
	"PDN Type",
	"Procedure Transaction ID",
	"DRX Parameter",
	"Reserved",
	"MM Context (GSM Key and Triplets)",
	"MM Context (UMTS KeyUsed Cipher and Quintuplets)",
	"MM Context (GSM KeyUsed Cipher and Quintuplets)",
	"MM Context (UMTS Key and Quintuplets)",
	"MM Context (EPS Security ContextQuadruplets and Quintuplets)",
	"MM Context (UMTS KeyQuadruplets and Quintuplets)",
	"PDN Connection",
	"PDU Numbers",
	"P-TMSI",
	"P-TMSI Signature",
	"Hop Counter",
	"UE Time Zone",
	"Trace Reference",
	"Complete Request Message",
	"GUTI",
	"F-Container",
	"F-Cause",
	"Selected PLMN ID",
	"Target Identification",
	"Reserved",
	"Packet Flow ID",
	"RAB Context",
	"Source RNC PDCP Context Info",
	"UDP Source Port Number",
	"APN Restriction",
	"Selection Mode",
	"Source Identification",
	"Reserved",
	"Change Reporting Action",
	"Fully Qualified PDN Connection Set Identifier (FQ-CSID)",
	"Channel needed",
	"eMLPP Priority",
	"Node Type",
	"Fully Qualified Domain Name (FQDN)",
	"Transaction Identifier (TI)",
	"MBMS Session",
	"MBMS Service Area",
	"MBMS Session Identifier",
	"MBMS Flow Identifier",
	"MBMS IP Multicast Distribution",
	"MBMS Distribution Acknowledge",
	"RFSP Index",
	"User CSG Information (UCI)",
	"CSG Information Reporting Action",
	"CSG ID",
	"CSG Membership Indication (CMI)",
	"Service indicator",
	"Detach Type",
	"Local Distiguished Name (LDN)",
	"Node Features",
	"MBMS Time to Data Transfer",
	"Throttling",
	"Allocation/Retention Priority (ARP)",
	"EPC Timer",
	"Signalling Priority Indication",
	"Temporary Mobile Group Identity (TMGI)",
	"Additional MM context for SRVCC",
	"Additional flags for SRVCC",
	"Max MBR/APN-AMBR (MMBR)",
	"MDT Configuration",
	"Additional Protocol Configuration Options (APCO)",
	"Unknown",	/* -  Spare. For future use.  */
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Unknown",
	"Private",
];


pub fn get_gtpv2_msg_type(t: u8) -> Result<&'static str, ()> {
    if t<0 || t >= GTPV2C_MSG_MAX {
		return Err(());
    }
    Ok(gtpv_msg_type_vals[t as usize])
}

pub fn get_gtpv2_ie_type(t: u8) -> Result<&'static str, ()> {
    if t < 0 || t >= GTPV2C_IE_TYPE_MAX {
		return Err(());
    }
    Ok(gtpv_ie_type_vals[t as usize])
}