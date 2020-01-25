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

pub fn get_port_in_range(range: PortRange) -> u16 {
    let mut port: u16 = range.min;

    for p in range.min..range.max {
        let port_available = check_port_available(&p);
        if port_available {
            port = p;
            break;
        }
    }

    return port;
}


pub fn get_port() -> u16 {
    let range = PortRange::default();
    let mut port: u16 = range.min;

    for i in range.min..range.max {
        let available = check_port_available(&i);
        if available {
            port = i;
        }
    }

    return port;
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
        let port = get_port_in_range(PortRange { min: 1337, max: 6000 } );
        assert_eq!(port, 1337);
    }

    #[test]
    fn test_gp() {
        let port = get_port();
        assert_ne!(port, 0);
    }
}