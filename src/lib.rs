
pub const USER_AGENT:&'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
pub const CLIENT_ID:&'static str = "zU4XHVVkc2tDPo4t";
pub const CLIENT_SECRET:&'static str = "VJKhDFqJPqvsPVNBV6ukXTJmwlvbttP7wlMlrc72se4=";
pub const API_BASE:&'static str = "https://api.tidalhifi.com/v1";
pub const AUTH_BASE:&'static str = "https://auth.tidal.com/v1/oauth2";

pub mod model;
pub mod client;
pub mod error;
pub mod auth;
pub mod media;
mod parsing;

use error::Error;
use model::*;