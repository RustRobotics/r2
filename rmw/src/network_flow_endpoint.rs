// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

use crate::ret_types;

const UNKNOWN: &str = "Unknown";

/// Transport protocol types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransportProtocol {
    Unknown = 0,
    Udp,
    Tcp,
    Count,
}

impl fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unknown => UNKNOWN,
            Self::Udp => "UDP",
            Self::Tcp => "TCP",
            Self::Count => UNKNOWN,
        };
        write!(f, "{}", s)
    }
}

/// Internet protocol types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InternetProtocol {
    Unknown = 0,
    Ipv4,
    Ipv6,
    Count,
}

impl fmt::Display for InternetProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unknown => UNKNOWN,
            Self::Ipv4 => "IPv4",
            Self::Ipv6 => "IPv6",
            Self::Count => UNKNOWN,
        };
        write!(f, "{}", s)
    }
}

/// Maximum length of internet address string including terminating null.
///
/// Inspired from linux/inet.h
pub const INET_ADDRSTRLEN: usize = 48;

/// Structure that describes network flow endpoint of a publisher or subscription.
#[derive(Debug, Clone, PartialEq)]
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
    pub internet_address: [u8; INET_ADDRSTRLEN],
}

impl NetworkFlowEndpoint {
    /// Return a NetworkFlowEndpoint struct with zero-initialized members.
    pub fn zero_inialized() -> Self {
        Self::default()
    }

    /// Set internet address.
    ///
    /// Returns `RET_OK` on successfull initilization,
    /// or returns `RET_INVALID_ARGUMENT` if `network_flow_endpoint` is NULL,
    /// or returns `RET_INVALID_ARGUMENT` if `internet_address` is NULL,
    /// or returns `RET_INVALID_ARGUMENT` if size of `internet_address` is
    /// more than `INET_ADDRSTRLEN`, or returns `RET_ERROR` when an unspecified error occurs.
    pub fn set_internet_address(&mut self, internet_address: &[u8]) -> ret_types::RetType {
        if internet_address.len() > INET_ADDRSTRLEN {
            log::error!("Size is not less than INET_ADDRSTRLEN");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        self.internet_address.fill(0);
        self.internet_address[0..internet_address.len()].copy_from_slice(internet_address);
        ret_types::RET_OK
    }
}

impl Default for NetworkFlowEndpoint {
    fn default() -> Self {
        Self {
            transport_protocol: TransportProtocol::Unknown,
            internet_protocol: InternetProtocol::Unknown,
            transport_port: 0,
            flow_label: 0,
            dscp: 0,
            internet_address: [0; INET_ADDRSTRLEN],
        }
    }
}
