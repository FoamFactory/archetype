use std::net::IpAddr;
use std::env;
use regex::Regex;

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

pub fn is_host_allowed(host: &IpAddr) -> bool {
    let allowed_host_re_string = match env::var("ARCHETYPE_ALLOWED_HOSTS") {
        Ok(v) => v,
        Err(_e) => "".to_string()
    };

    if allowed_host_re_string.is_empty() {
        return false;
    }

    let allowed_host_re_result = Regex::new(&allowed_host_re_string);
    if allowed_host_re_result.is_err() {
        println!("ALLOWED_HOSTS doesn't appear to be a valid regular expression: {}", allowed_host_re_string);
        return false;
    }

    let allowed_host_re = allowed_host_re_result.unwrap();

    return allowed_host_re.is_match(&host.to_string());
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::net::{IpAddr, Ipv4Addr};
    use crate::util::{get_allowed_hosts_from_environment, is_host_allowed};

    #[test]
    fn it_should_show_that_no_hosts_are_allowed() {
        env::set_var("ARCHETYPE_ALLOWED_HOSTS", "");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let host_allowed = is_host_allowed(&ip_addr);
        assert!(!host_allowed);
    }

    #[test]
    fn it_should_show_a_single_allowed_host() {
        env::set_var("ARCHETYPE_ALLOWED_HOSTS", "127.0.0.1");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let host_allowed = is_host_allowed(&ip_addr);
        assert!(host_allowed);
    }

    #[test]
    fn it_should_allow_all_hosts_starting_with_172() {
        env::set_var("ARCHETYPE_ALLOWED_HOSTS", "172\\.([0-9]{1,3})\\.([0-9]{1,3})\\.([0-9]{1,3})");
        let socket: IpAddr = IpAddr::V4(Ipv4Addr::new(172, 168, 202, 1));
        assert!(is_host_allowed(&socket));
    }
}