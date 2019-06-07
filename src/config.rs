//! configuration file handling module
//! This module provides structure and methods related to configuration file handling.
use super::*;

// std
use std::net::IpAddr;

/// Main Configuration Structure
#[derive(Debug, Deserialize)]
pub struct CConfig {
    pub verbose: Option<u8>,
    pub debug: Option<u8>,
    pub pid: Option<String>,
    pub working_dir: Option<String>,
    pub main_log: Option<String>,
    pub error_log: Option<String>,
    pub vrouter: Option<Vec<VRConfig>>,
    pub protocols: Option<Protocols>,
}

impl CConfig {
    // verbose() getter
    pub fn _verbose(&self) -> Option<u8> {
        self.verbose
    }
    // debug() getter
    pub fn debug(&self) -> u8 {
        match self.debug {
            Some(v) => v,
            None => DEBUG_LEVEL_NONE,
        }
    }
    // pid() getter
    pub fn pid(&self) -> String {
        match &self.pid {
            Some(v) => v.clone(),
            None => RVRRPD_DFLT_PIDFILE.to_string(),
        }
    }
    // working_dir() getter
    pub fn working_dir(&self) -> String {
        match &self.working_dir {
            Some(v) => v.clone(),
            None => RVRRPD_DFLT_WORKDIR.to_string(),
        }
    }
    // main_log() getter
    pub fn main_log(&self) -> String {
        match &self.main_log {
            Some(v) => v.clone(),
            None => RVRRPD_DFLT_LOGFILE.to_string(),
        }
    }
    // error_log() getter
    pub fn error_log(&self) -> String {
        match &self.error_log {
            Some(v) => v.clone(),
            None => RVRRPD_DFLT_ELOGFILE.to_string(),
        }
    }
}

/// Virtual-Routers Configuration Structure
#[derive(Debug, Deserialize)]
pub struct VRConfig {
    group: u8,
    interface: String,
    vip: Option<String>,
    priority: Option<u8>,
    preemption: Option<bool>,
    auth_type: Option<u8>,
    auth_secret: Option<String>,
    timers: Option<Timers>,
    rfc3768: Option<bool>,
}
impl VRConfig {
    // group() getter
    pub fn group(&self) -> u8 {
        if self.group < 1 {
            panic!("error(config): Please configure a group id between 1 and 255")
        }
        self.group
    }
    // interface() getter
    pub fn interface(&self) -> &String {
        &self.interface
    }
    // vip() getter
    pub fn vip(&self) -> [u8; 4] {
        match &self.vip {
            Some(ip) => match ip.parse::<IpAddr>().unwrap() {
                IpAddr::V4(ip) => ip.octets(),
                IpAddr::V6(_ipv6) => panic!("error(config) Only IPv4 addresses are supported"),
            },
            None => panic!("error(config) No virtual IP specified"),
        }
    }
    // timer_advert() getter
    pub fn timer_advert(&self) -> u8 {
        match &self.timers {
            Some(t) => t.advert,
            None => 1,
        }
    }
    // priority() getter
    pub fn priority(&self) -> u8 {
        match self.priority {
            Some(v) => {
                if v < 1 || v > 254 {
                    panic!("error(config) Please configure a priority between 1 and 254");
                }
                v
            }
            None => VRRP_V2_DEFAULT_PRIORITY,
        }
    }
    // preemption() getter
    pub fn preemption(&self) -> bool {
        match self.preemption {
            Some(b) => b,
            None => false,
        }
    }
    // auth_type() getter
    pub fn auth_type(&self) -> u8 {
        match self.auth_type {
            Some(v) => match v {
                0 => v,
                _ => panic!(
                    "error(config) Only authentication type 0 (plain) is currently supported"
                ),
            },
            None => 0,
        }
    }
    // auth_secret() getter
    pub fn auth_secret(&self) -> &Option<String> {
        &self.auth_secret
    }
    // rfc3768() getter
    pub fn rfc3768(&self) -> bool {
        match self.rfc3768 {
            Some(b) => b,
            None => true,
        }
    }
}

/// Timers Option Type
#[derive(Debug, Deserialize)]
struct Timers {
    advert: u8,
}
impl Default for Timers {
    fn default() -> Self {
        Timers { advert: 1 }
    }
}

/// Protocols Option Type
#[derive(Debug, Deserialize)]
pub struct Protocols {
    pub r#static: Option<Vec<Static>>,
}

/// Static Option Type
#[derive(Debug, Deserialize)]
pub struct Static {
    route: String,
    mask: String,
    nh: String,
    metric: Option<u16>,
    mtu: Option<u16>,
}

// Static Option Implementation
impl Static {
    // route() getter
    // convert IPv4 String to array of four 8-bits unsigned integers
    pub fn route(&self) -> [u8; 4] {
        match self.route.parse::<IpAddr>().unwrap() {
            IpAddr::V4(ip) => ip.octets(),
            IpAddr::V6(_ipv6) => panic!("error(config-static) Only IPv4 routes are supported"),
        }
    }
    // mask() getter
    pub fn mask(&self) -> [u8; 4] {
        match self.mask.parse::<IpAddr>().unwrap() {
            IpAddr::V4(ip) => ip.octets(),
            IpAddr::V6(_ipv6) => panic!("error(config-static) Only IPv4 masks are supported"),
        }
    }
    // nh() getter
    pub fn nh(&self) -> [u8; 4] {
        match self.nh.parse::<IpAddr>().unwrap() {
            IpAddr::V4(ip) => ip.octets(),
            IpAddr::V6(_ipv6) => panic!("error(config-static) Only IPv4 next-hops are supported"),
        }
    }
    // metric() getter
    pub fn metric(&self) -> i16 {
        match self.metric {
            Some(v) => v as i16,
            None => 0,
        }
    }
    // mtu() getter
    pub fn mtu(&self) -> u64 {
        match self.mtu {
            Some(v) => v as u64,
            None => 0,
        }
    }
}

// decode_config() function
/// read and decode configuration file
pub fn decode_config(filename: String) -> CConfig {
    let file = std::fs::read_to_string(filename).expect("Cannot read rVRRPd configuration file");
    let config: CConfig = match toml::from_str(&file) {
        Ok(c) => c,
        Err(e) => panic!("error(config): Cannot parse configuration file:\n {}", e),
    };
    // return config
    config
}
