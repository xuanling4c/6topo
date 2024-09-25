mod new;
mod execute;


use std::sync::Arc;
use crate::core::conf::set_conf::base_conf::BaseConf;
use crate::core::conf::set_conf::receiver_conf::ReceiverBaseConf;
use crate::core::conf::set_conf::sender_conf::SenderBaseConf;
use crate::modes::{Helper};
use crate::modules::probe_modules::probe_mod_v6::ProbeModV6;
use crate::modules::target_iterators::{CycleIpv6Type};
use crate::SYS;
use crate::tools::blocker::ipv6_blocker::BlackWhiteListV6;

/// zmap_v6
pub struct CycleV6 {
    pub base_conf:Arc<BaseConf>,
    pub target_iter:CycleIpv6Type,
    pub sender_conf:Arc<SenderBaseConf>,
    pub receiver_conf:Arc<ReceiverBaseConf>,

    pub probe:Arc<ProbeModV6>,

    pub start_ip:u128,
    pub end_ip:u128,
    pub tar_ip_num:u64,

    pub ttl:Option<u8>,

    pub assigned_target_range:Vec<(u128,u128,u64)>,
    pub blocker:BlackWhiteListV6,
}


impl  Helper  for CycleV6 {
    fn print_help() -> String {

        SYS.get_info("help", "CycleV6")
    }
}