// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Transport protocol types
#[repr(u8)]
#[derive(Debug)]
pub enum TransportProtocol {
    Unknown = 0,
    Udp,
    Tcp,
    Count,
}

/// Internet protocol types
#[repr(u8)]
#[derive(Debug)]
pub enum InternetProtocol {
    Unknown = 0,
    Ipv4,
    Ipv6,
    Count,
}

/// Maximum length of internet address string including terminating null
/// Inspired from linux/inet.h
pub const RMW_INET_ADDRSTRLEN: usize = 48;

/// Structure that describes network flow endpoint of a publisher or subscription
#[derive(Debug)]
pub struct NetworkFlowEndpoint {
    /// Transport protocol
    pub transport_protocol: TransportProtocol,

    /// Internet protocol
    pub internet_protocol: InternetProtocol,

    /// Port
    pub transport_port: u16,

    /// Flow label
    pub flow_label: u32,

    /// DSCP (Diff. Services Code Point)
    pub dscp: u8,

    /// Internet address
    pub internet_address: [u8; RMW_INET_ADDRSTRLEN],
}
