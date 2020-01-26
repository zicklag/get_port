use std::default::Default;
use std::net::{ TcpListener, Ipv4Addr };

#[derive(Debug)]
pub struct PortRange {
    min: u16,
    max: u16
}

impl Default for PortRange {
    fn default() -> Self {
        PortRange { min: 1024, max: 65535 }
    }
}

pub fn get_port_in_range(range: PortRange) -> Option<u16> {
    return (range.min..range.max).filter(|p| check_port_available(&p)).nth(0); 
}


pub fn get_port() -> Option<u16> {
    return get_port_in_range(PortRange::default());
}

fn check_port_available(port: &u16) -> bool {
    match TcpListener::bind((Ipv4Addr::LOCALHOST, *port)) {
        Ok(_) => return true,
        Err(_) => return false
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gpir() {
        let port = get_port_in_range(PortRange { min: 1337, max: 6000 } ).unwrap();
        assert_eq!(port, 1337);
    }

    #[test]
    fn test_gp() {
        let port = get_port().unwrap();
        assert_ne!(port, 0);
    }
}