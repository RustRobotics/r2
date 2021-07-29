// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::init::Context;

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

/// R2 graph ID of the topic.
#[derive(Debug)]
pub struct Gid {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Byte data Gid value
    pub data: [u8; GID_STORAGE_SIZE],
}
