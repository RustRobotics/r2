// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::init::Context;
use super::time::TimePointValue;

/// 24 bytes is the most memory needed to represent the GID by any current
/// implementation. It may need to be increased in the future.
pub const GID_STORAGE_SIZE: usize = 24;

/// Structure which encapsulates an rmw node
pub struct Node {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this node's data
    //void * data;

    /// A concise name of this rmw node for identification
    pub name: String,

    /// The namespace of this rmw node
    pub namespace_: String,

    /// Context information about node's init specific information
    pub context: Box<Context>,
}

/// Endpoint enumeration type
#[repr(u8)]
#[derive(Debug)]
pub enum EndpointType {
    /// Endpoint type has not yet been set
    Invalid = 0,

    /// Creates and publishes messages to the ROS topic
    Publisher,

    /// Listens for and receives messages from a topic
    Subscription,
}

/// Information describing an rmw message
#[derive(Debug)]
pub struct MessageInfo {
    pub source_timestamp: TimePointValue,
    pub received_timestamp: TimePointValue,
    pub publisher_gid: Gid,

    /// Whether this message is from intra_process communication or not
    pub from_intra_process: bool,
}

/// R2 graph ID of the topic.
#[derive(Debug)]
pub struct Gid {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Byte data Gid value
    pub data: [u8; GID_STORAGE_SIZE],
}
