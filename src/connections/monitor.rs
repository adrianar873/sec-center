use procfs::net::{tcp, tcp6, udp, udp6};
use std::net::SocketAddr;

/// Represents an active network connection
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Connection {
    pub local_addr: SocketAddr,
    pub remote_addr: Option<SocketAddr>,
    pub protocol: String,      // "TCP" or "UDP"
    pub state: Option<String>,    // TCP state (ESTABLISHED, LISTEN, etc.)
    pub inode: u64,            // For process lookup (future use)
}

/// Reads active TCP connections from /proc/net/tcp and tcp6
pub fn get_tcp_connections() -> Vec<Connection> {
    let mut connections = Vec::new();
    
    // Read TCP (IPv4)
    if let Ok(entries) = self::tcp() {
        for entry in entries {
            let state_str = format!("{:?}", entry.state);
            connections.push(Connection {
                local_addr: entry.local_address,
                remote_addr: if entry.remote_address.ip().is_unspecified() {
                    None
                } else {
                    Some(entry.remote_address)
                },
                protocol: "TCP".to_string(),
                state: Some(state_str),
                inode: entry.inode,
            });
        }
    }
    
    // Read TCP6 (IPv6)
    if let Ok(entries) = self::tcp6() {
        for entry in entries {
            let state_str = format!("{:?}", entry.state);
            connections.push(Connection {
                local_addr: entry.local_address,
                remote_addr: if entry.remote_address.ip().is_unspecified() {
                    None
                } else {
                    Some(entry.remote_address)
                },
                protocol: "TCP6".to_string(),
                state: Some(state_str),
                inode: entry.inode,
            });
        }
    }
    
    connections
}

/// Reads active UDP sockets from /proc/net/udp and udp6
pub fn get_udp_sockets() -> Vec<Connection> {
    let mut connections = Vec::new();
    
    // Read UDP (IPv4)
    if let Ok(entries) = self::udp() {
        for entry in entries {
            connections.push(Connection {
                local_addr: entry.local_address,
                remote_addr: None,  // UDP is connectionless
                protocol: "UDP".to_string(),
                state: None,
                inode: entry.inode,
            });
        }
    }
    
    // Read UDP6 (IPv6)
    if let Ok(entries) = self::udp6() {
        for entry in entries {
            connections.push(Connection {
                local_addr: entry.local_address,
                remote_addr: None,
                protocol: "UDP6".to_string(),
                state: None,
                inode: entry.inode,
            });
        }
    }
    
    connections
}
