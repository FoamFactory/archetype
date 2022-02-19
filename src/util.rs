use std::net::IpAddr;
use std::env;

use crate::models::Avatar;

pub fn get_version_code_from_string(version_string: &str) -> u32 {
    let mut multiplier: u32 = 1;
    let tokens: Vec<u32> = version_string
        .split(".")
        .map(|s| String::from(s))
        .map(|s| {
            multiplier = multiplier + 2;
            multiplier * (s.parse::<u32>().unwrap())
        })
        .collect();
    tokens.into_iter().sum()
}

pub fn get_data_uri_for_avatar(avatar: &Avatar) -> String {
    format!("data:{};base64,{}", avatar.mimetype, avatar.image)
}

pub fn get_allowed_hosts_from_environment() -> Vec<IpAddr> {
    let ah_string = match env::var("ARCHETYPE_ALLOWED_HOSTS") {
        Ok(v) => v,
        Err(_e) => "".to_string()
    };

    let allowed_hosts_str: Vec<String> = ah_string.split(",").map(|s| s.to_string()).collect();
    let allowed_hosts: Vec<IpAddr> = allowed_hosts_str.into_iter()
        .map(|ah| ah.parse::<IpAddr>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();

    allowed_hosts
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::net::{IpAddr, Ipv4Addr};
    use crate::util::get_allowed_hosts_from_environment;

    #[test]
    fn it_should_show_that_no_hosts_are_allowed() {
        let empty_vec: Vec<IpAddr> = vec![];
        assert_eq!(empty_vec, get_allowed_hosts_from_environment());
    }

    #[test]
    fn it_should_show_a_single_allowed_host() {
        env::set_var("ARCHETYPE_ALLOWED_HOSTS", "127.0.0.1");
        let socket_vec = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        assert_eq!(socket_vec, get_allowed_hosts_from_environment());
    }
}