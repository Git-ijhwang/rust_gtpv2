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
const GTPV2C_ECHO_REQ: i32 = 		                            1;
const GTPV2C_ECHO_RSP: i32 = 		                            2;
const GTPV2C_VERSION_NOT_SUPPORTED_IND: i32 = 		            3;
 		/* ----------------------------------------- *
 		 * SGSN/MME/ePDG to PGW (S4/S11, S5/S8, S2b) *
 		 * ----------------------------------------- */
const GTPV2C_CREATE_SESSION_REQ: i32 = 		                    32;
const GTPV2C_CREATE_SESSION_RSP: i32 = 		                    33;
const GTPV2C_DELETE_SESSION_REQ: i32 = 		                    36;
const GTPV2C_DELETE_SESSION_RSP: i32 = 		                    37;
 		/* ------------------------------- *
 		 * SGSN/MME to PGW (S4/S11, S5/S8) *
 		 * ------------------------------- */
const GTPV2C_MODIFY_BEARER_REQ: i32 = 		                    34;
const GTPV2C_MODIFY_BEARER_RSP: i32 = 		                    35;
const GTPV2C_CHANGE_NTF_REQ: i32 = 		                        38;
const GTPV2C_CHANGE_NTF_RSP: i32 = 		                        39;
const GTPV2C_RESUME_NTF: i32 = 		                            164;
const GTPV2C_RESUME_ACK: i32 = 		                            165;
 		/* ---------------------------------- *
 		 * Messages without explicit response *
 		 * ---------------------------------- */
const GTPV2C_MODIFY_BEARER_CMD: i32 = 		                    64; 		/* MME/SGSN/ePDG to PGW - S11/S4, S5/S8, S2b */
const GTPV2C_MODIFY_BEARER_FAILURE_IND: i32 = 		            65; 		/* PGW to MME/SGSN/ePDG - S5/S8, S11/S4, S2b */
const GTPV2C_DELETE_BEARER_CMD: i32 = 		                    66; 		/* MME/SGSN to PGW - S11/S4, S5/S8 */
const GTPV2C_DELETE_BEARER_FAILURE_IND: i32 = 		            67; 		/* PGW to MME/SGSN -S5/S8, S11/S4 */
const GTPV2C_BEARER_RESOURCE_CMD: i32 = 		                68; 		/* MME/SGSN to PGW - S11/S4, S5/S8 */
const GTPV2C_BEARER_RESOURCE_FAILURE_IND: i32 = 	            69; 		/* PGW to MME/SGSN -S5/S8, S11/S4 */
const GTPV2C_DOWNLINK_DATA_NOTIFICATION_FAILURE_IND: i32 = 		70; 		/* SGSN/MME to SGW - S4/S11 */
const GTPV2C_TRACE_SESSION_ACTIVATION: i32 = 		            71; 		/* MME/SGSN/ePDG to PGW - S11/S4, S5/S8, S2b */
const GTPV2C_TRACE_SESSION_DEACTIVATION: i32 = 		            72; 		/* MME/SGSN/ePDG to PGW - S11/S4, S5/S8, S2b */
const GTPV2C_STOP_PAGING_IND: i32 = 		                    73; 		/* SGW to MME/SGSN - S11/S4 */
 		/* ----------------------------------------- *
 		 * PGW to SGSN/MME/ePDG (S5/S8, S4/S11, S2b) *
 		 * ----------------------------------------- */
const GTPV2C_CREATE_BEARER_REQ: i32 = 		                    95;
const GTPV2C_CREATE_BEARER_RSP: i32 = 		                    96;
const GTPV2C_UPDATE_BEARER_REQ: i32 = 		                    97;
const GTPV2C_UPDATE_BEARER_RSP: i32 = 		                    98;
const GTPV2C_DELETE_BEARER_REQ: i32 = 		                    99;
const GTPV2C_DELETE_BEARER_RSP: i32 = 		                    100;
 		/* ------------------------------------------------------------------------------------------ *
 		 * PGW to MME, MME to PGW, SGW to PGW, SGW to MME, PGW to ePDG, ePDG to PGW (S5/S8, S11, S2b) *
 		 * ------------------------------------------------------------------------------------------ */
const GTPV2C_DELETE_PDN_CONNECTION_SET_REQ: i32 = 		        101;
const GTPV2C_DELETE_PDN_CONNECTION_SET_RSP: i32 = 		        102;
 		/* --------------------------------------------------------------- *
 		 * MME to MME, SGSN to MME, MME to SGSN, SGSN to SGSN (S3/S10/S16) *
 		 * --------------------------------------------------------------- */
const GTPV2C_IDENTIFICATION_REQ: i32 = 		                    128;
const GTPV2C_IDENTIFICATION_RSP: i32 = 		                    129;
const GTPV2C_CONTEXT_REQ: i32 = 		                        130;
const GTPV2C_CONTEXT_RSP: i32 = 		                        131;
const GTPV2C_CONTEXT_ACK: i32 = 		                        132;
const GTPV2C_FORWARD_RELOCATION_REQ: i32 = 		                133;
const GTPV2C_FORWARD_RELOCATION_RSP: i32 = 		                134;
const GTPV2C_FORWARD_RELOCATION_COMPLETE_NTF: i32 = 		    135;
const GTPV2C_FORWARD_RELOCATION_COMPLETE_ACK: i32 = 		    136;
const GTPV2C_FORWARD_ACCESS_CONTEXT_NTF: i32 = 		            137;
const GTPV2C_FORWARD_ACCESS_CONTEXT_ACK: i32 = 		            138;
const GTPV2C_RELOCATION_CANCEL_REQ: i32 = 		                139;
const GTPV2C_RELOCATION_CANCEL_RSP: i32 = 		                140;
const GTPV2C_CONFIGURE_TRANSFER_TUNNEL: i32 = 		            141;
const GTPV2C_RAN_INFORMATION_RELAY: i32 = 		                152;
 		/* ----------------------------- *
 		 * SGSN to MME, MME to SGSN (S3) *
 		 * ----------------------------- */
const GTPV2C_DETACH_NTF: i32 = 		                            149;
const GTPV2C_DETACH_ACK: i32 = 		                            150;
const GTPV2C_CS_PAGING_INDICATION: i32 = 		                151;
const GTPV2C_ALERT_MME_NTF: i32 = 		                        153;
const GTPV2C_ALERT_MME_ACK: i32 = 		                        154;
const GTPV2C_UE_ACTIVITY_NTF: i32 = 		                    155;
const GTPV2C_UE_ACTIVITY_ACK: i32 = 		                    156;
 		/* ---------------------------------------- *
 		 * SGSN/MME to SGW, SGSN to MME (S4/S11/S3) *
         * SGSN to SGSN (S16), SGW to PGW (S5/S8)   *
 		 * ---------------------------------------- */
const GTPV2C_SUSPEND_NTF: i32 = 		                        162;
const GTPV2C_SUSPEND_ACK: i32 = 		                        163;
 		/* ------------------------ *
 		 * SGSN/MME to SGW (S4/S11) *
 		 * ------------------------ */
const GTPV2C_CREATE_FORWARDING_TUNNEL_REQ: i32 = 		        160;
const GTPV2C_CREATE_FORWARDING_TUNNEL_RSP: i32 = 		        161;
const GTPV2C_CREATE_INDIRECT_DATA_FORWARDING_TUNNEL_REQ: i32 = 		166;
const GTPV2C_CREATE_INDIRECT_DATA_FORWARDING_TUNNEL_RSP: i32 = 		167;
const GTPV2C_DELETE_INDIRECT_DATA_FORWARDING_TUNNEL_REQ: i32 = 		168;
const GTPV2C_DELETE_INDIRECT_DATA_FORWARDING_TUNNEL_RSP: i32 = 		169;
const GTPV2C_RELEASE_ACCESS_BEARERS_REQ: i32 = 		            170;
const GTPV2C_RELEASE_ACCESS_BEARERS_RSP: i32 = 		            171;
 		/* ------------------------ *
 		 * SGW to SGSN/MME (S4/S11) *
 		 * ------------------------ */
const GTPV2C_DOWNLINK_DATA_NOTIFICATION: i32 = 		            176;
const GTPV2C_DOWNLINK_DATA_NOTIFICATION_ACK: i32 = 		        177;
const GTPV2C_PGW_RESTART_NOTIFICATION: i32 = 		            179;
const GTPV2C_PGW_RESTART_NOTIFICATION_ACK: i32 = 		        180;
 		/* ------------------------------ *
 		 * SGW to PGW, PGW to SGW (S5/S8) *
 		 * ------------------------------ */
const GTPV2C_UPDATE_PDN_CONNECTION_SET_REQ: i32 = 		        200;
const GTPV2C_UPDATE_PDN_CONNECTION_SET_RSP: i32 = 		        201;
 		/* ---------------- *
 		 * MME to SGW (S11) *
 		 * ---------------- */
const GTPV2C_MODIFY_ACCESS_BEARERS_REQ: i32 = 		            211;
const GTPV2C_MODIFY_ACCESS_BEARERS_RSP: i32 = 		            212;
 		/* --------------------------- *
 		 * MBMS GW to MME/SGSN (Sm/Sn) *
 		 * --------------------------- */
const GTPV2C_MBMS_SESSION_START_REQ: i32 = 		                231;
const GTPV2C_MBMS_SESSION_START_RSP: i32 = 		                232;
const GTPV2C_MBMS_SESSION_UPDATE_REQ: i32 = 		            233;
const GTPV2C_MBMS_SESSION_UPDATE_RSP: i32 = 		            234;
const GTPV2C_MBMS_SESSION_STOP_REQ: i32 = 		                235;
const GTPV2C_MBMS_SESSION_STOP_RSP: i32 = 		                236;
const GTPV2C_MSG_MAX: i32 = 		                            256;


/*
 * GTPv2-C Information Element Type Values
 * ======================================================
 */
const GTPV2C_IE_RESERVED: i32 = 		                        0;
const GTPV2C_IE_IMSI: i32 = 		                            1; 		/* International Mobile Subscriber Identity */
const GTPV2C_IE_CAUSE: i32 = 		                            2;
const GTPV2C_IE_RECOVERY: i32 = 		                        3; 		/* Restart Counter */
 		/* Reserved for S101 interface 		4 to 50 */
const GTPV2C_IE_STN_SR: i32 = 		                            51;
 		/* Reserved for Sv interface 		52 to 70 */
const GTPV2C_IE_APN: i32 = 		                                71; 		/* Access Point Name */
const GTPV2C_IE_AMBR: i32 = 		                            72; 		/* Aggregate Maximum Bit Rate */
const GTPV2C_IE_EBI: i32 = 		                                73; 		/* EPS Bearer ID */
const GTPV2C_IE_IP_ADDRESS: i32 = 		                        74;
const GTPV2C_IE_MEI: i32 = 		                                75; 		/* Mobile Equipment Identity */
const GTPV2C_IE_MSISDN: i32 = 		                            76;
const GTPV2C_IE_INDICATION: i32 = 		                        77;
const GTPV2C_IE_PCO: i32 = 		                                78; 		/* Protocol Configuration Options */
const GTPV2C_IE_PAA: i32 = 		                                79; 		/* PDN Address Allocation */
const GTPV2C_IE_BEARER_QOS: i32 = 		                        80; 		/* Bearer Level Quality of Service */
const GTPV2C_IE_FLOW_QOS: i32 = 		                        81; 		/* Flow Quality of Service */
const GTPV2C_IE_RAT_TYPE: i32 = 		                        82;
const GTPV2C_IE_SERVING_NETWORK: i32 = 		                    83;
const GTPV2C_IE_BEARER_TFT: i32 = 		                        84; 		/* EPS Bearer Level Traffic Flow Template */
const GTPV2C_IE_TAD: i32 = 		                                85; 		/* Traffic Aggregation Description */
const GTPV2C_IE_ULI: i32 = 		                                86; 		/* User Location Information */
const GTPV2C_IE_FTEID: i32 = 		                            87; 		/* Fully Qualified Tunnel Endpoint Identifier */
const GTPV2C_IE_TMSI: i32 = 		                            88;
const GTPV2C_IE_GLOBAL_CN_ID: i32 = 		                    89;
const GTPV2C_IE_S103PDF: i32 = 		                            90; 		/* S103 PDN Data Forwarding Info */
const GTPV2C_IE_S1UDF: i32 = 		                            91; 		/* S1-U Data Forwarding Info */
const GTPV2C_IE_DELAY_VALUE: i32 = 		                        92;
const GTPV2C_IE_BEARER_CONTEXT: i32 = 		                    93;
const GTPV2C_IE_CHARGING_ID: i32 = 		                        94;
const GTPV2C_IE_CHARGING_CHARACTERISTICS: i32 = 		        95;
const GTPV2C_IE_TRACE_INFORMATION: i32 = 		                96;
const GTPV2C_IE_BEARER_FLAGS: i32 = 		                    97;
 		/* Reserved: i32 = 		98 */
const GTPV2C_IE_PDN_TYPE: i32 = 		                        99;
const GTPV2C_IE_PROCEDURE_TRANSACTION_ID: i32 = 		        100;
const GTPV2C_IE_DRX_PARAMETER: i32 = 		                    101;
 		/* Reserved 		102 */
 		/* MM Context 		103 to 108 */
const GTPV2C_IE_PDN_CONNECTION: i32 = 		                    109;
const GTPV2C_IE_PDU_NUMBERS: i32 = 		                        110;
const GTPV2C_IE_PTMSI: i32 = 		                            111;
const GTPV2C_IE_PTMSI_SIGNATURE: i32 = 		                    112;
const GTPV2C_IE_HOP_COUNTER: i32 = 		                        113;
const GTPV2C_IE_UE_TIME_ZONE: i32 = 		                    114;
const GTPV2C_IE_TRACE_REFERENCE: i32 = 		                    115;
const GTPV2C_IE_COMPLETE_REQUEST_MESSAGE: i32 = 		        116;
const GTPV2C_IE_GUTI: i32 = 		                            117;
const GTPV2C_IE_F_CONTAINER: i32 = 		                        118;
const GTPV2C_IE_F_CAUSE: i32 = 		                            119;
const GTPV2C_IE_SELECTED_PLMN_ID: i32 = 		                120;
const GTPV2C_IE_TARGET_IDENTIFICATION: i32 = 		            121;
 		/* Reserved 		122 */
const GTPV2C_IE_PACKET_FLOW_ID: i32 = 		                    123;
const GTPV2C_IE_RAB_CONTEXT: i32 = 		                        124;
const GTPV2C_IE_SOURCE_RNC_PDCP_CONTEXT_INFO: i32 = 		    125;
const GTPV2C_IE_UDP_SOURCE_PORT_NUMBER: i32 = 		            126;
const GTPV2C_IE_APN_RESTRICTION: i32 =                     		127;
const GTPV2C_IE_SELECTION_MODE: i32 = 		                    128;
const GTPV2C_IE_SOURCE_IDENTIFICATION: i32 = 		            129;
 		/* Reserved 		130 */
const GTPV2C_IE_CHANGE_REPORTING_ACTION: i32 = 		            131;
const GTPV2C_IE_FQ_CSID: i32 = 		                            132; 		/* Fully Qualified PDN Connection Set Identifier */
const GTPV2C_IE_CHANNEL_NEEDED: i32 = 		                    133;
const GTPV2C_IE_EMLPP_PRIORITY: i32 = 		                    134;
const GTPV2C_IE_NODE_TYPE: i32 = 		                        135;
const GTPV2C_IE_FQDN: i32 = 		                            136; 		/* Fully Qualified Domain Name */
const GTPV2C_IE_TI: i32 = 		                                137; 		/* Transaction Identifier */
const GTPV2C_IE_MBMS_SESSION_DURATION: i32 = 		            138;
const GTPV2C_IE_MBMS_SERIVCE_AREA: i32 = 		                139;
const GTPV2C_IE_MBMS_SESSION_IDENTIFIER: i32 = 		            140;
const GTPV2C_IE_MBMS_FLOW_IDENTIFIER: i32 = 		            141;
const GTPV2C_IE_MBMS_IP_MULTICAST_DISTRIBUTION: i32 = 		    142;
const GTPV2C_IE_MBMS_DISTRIBUTION_ACK: i32 = 		            143;
const GTPV2C_IE_RFSP_INDEX: i32 = 		                        144;
const GTPV2C_IE_UCI: i32 = 		                                145; 		/* User CSG Information */
const GTPV2C_IE_CSG_INFORMATION_REPORTING_ACTION: i32 = 		146;
const GTPV2C_IE_CSG_ID: i32 = 		                            147;
const GTPV2C_IE_CMI: i32 = 		                                148; 		/* CSG Membership Indication */
const GTPV2C_IE_SERVICE_INDICATOR: i32 = 		                149;
const GTPV2C_IE_DETACH_TYPE: i32 = 		                        150;
const GTPV2C_IE_LDN: i32 = 		                                151; 		/* Local Distiguished Name */
const GTPV2C_IE_NODE_FEATURES: i32 = 		                    152;
const GTPV2C_IE_MBMS_TIME_TO_DATA_TRANSFER: i32 = 		        153;
const GTPV2C_IE_THROTTING: i32 = 		                        154;
const GTPV2C_IE_ARP: i32 = 		                                155; 		/* Allocation/Retention Priority */
const GTPV2C_IE_EPC_TIMER: i32 = 		                        156;
const GTPV2C_IE_SIGNALLING_PRIORITY_INDICATION: i32 =     		157;
const GTPV2C_IE_TMGI: i32 = 		                            158; 		/* Temporary Mobile Group Identity */
const GTPV2C_IE_ADDITIONAL_MM_CONTEXT_FOR_SRVCC: i32 =     		159;
const GTPV2C_IE_ADDITIONAL_FLAGS_FOR_SRVCC: i32 = 		        160;
const GTPV2C_IE_MMBR: i32 = 		                            161; 		/* Max MBR/APN-AMBR */
const GTPV2C_IE_MDT_CONFIGURATION: i32 = 		                162;
const GTPV2C_IE_APCO: i32 = 		                            163; 		/* Additional Protocol Configuration Options */
 		/* Spare. For future use. 		164 to 254 */
const GTPV2C_IE_PRIVATE_EXTENSION: i32 = 		                255;
const GTPV2C_IE_TYPE_MAX: i32 = 		                        256;




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


pub fn get_gtpv2_msg_type(t: i32) -> Result<&'static str, ()> {
    if t<0 || t >= GTPV2C_MSG_MAX {
		return Err(());
    }
    Ok(gtpv_msg_type_vals[t as usize])
}

pub fn get_gtpv2_ie_type(t: i32) -> Result<&'static str, ()> {
    if t < 0 || t >= GTPV2C_IE_TYPE_MAX {
		return Err(());
    }
    Ok(gtpv_ie_type_vals[t as usize])
}