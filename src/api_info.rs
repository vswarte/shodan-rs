use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ApiInfoResponse {
    pub scan_credits: u32,
    pub usage_limits: ApiInfoResponseUsageLimits,
    pub plan: String,
    pub https: bool,
    pub unlocked: bool,
    pub query_credits: u32,
    pub monitored_ips: Option<u32>,
    pub unlocked_left: u32,
    pub telnet: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApiInfoResponseUsageLimits {
    pub scan_credits: i32,
    pub query_credits: i32,
    pub monitored_ips: i32,
}