// Connections module - handles active connection monitoring
//
// This module reads from /proc/net/tcp, tcp6, udp, and udp6
// to display active network connections and sockets.
// No root privileges required (uses procfs crate).

pub mod monitor;

use monitor::Connection;
use std::time::Instant;

/// Main connection monitoring state
///
/// Tracks active TCP/UDP connections and updates periodically.
#[allow(dead_code)]
pub struct ConnectionInfo {
    /// List of all active connections (TCP + UDP)
    pub connections: Vec<Connection>,
    /// Count of active TCP connections
    pub tcp_count: usize,
    /// Count of active UDP sockets
    pub udp_count: usize,
    /// Timestamp of last refresh
    pub last_update: Instant,
}

#[allow(dead_code)]
impl ConnectionInfo {
    /// Creates a new ConnectionInfo instance with empty state
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            tcp_count: 0,
            udp_count: 0,
            last_update: Instant::now(),
        }
    }

    /// Refreshes connection list from /proc/net
    ///
    /// Reads TCP (IPv4/IPv6) and UDP (IPv4/IPv6) connection tables.
    /// Updates counts and timestamp.
    pub fn refresh(&mut self) {
        let mut all_connections = Vec::new();
        
        // Get TCP connections (IPv4 + IPv6)
        let tcp_conns = monitor::get_tcp_connections();
        let tcp_count = tcp_conns.len();
        all_connections.extend(tcp_conns);
        
        // Get UDP sockets (connectionless)
        let udp_conns = monitor::get_udp_sockets();
        let udp_count = udp_conns.len();
        all_connections.extend(udp_conns);
        
        self.connections = all_connections;
        self.tcp_count = tcp_count;
        self.udp_count = udp_count;
        self.last_update = Instant::now();
    }
}
