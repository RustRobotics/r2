// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::ret_types::RetType;

/// 24 bytes is the most memory needed to represent the GID by any current
/// implementation. It may need to be increased in the future.
pub const GID_STORAGE_SIZE: usize = 24;

/// R2 graph ID of the topic.
#[derive(Debug)]
pub struct Gid {
    /// Name of the rmw implementation
    pub implementation_identifier: String,

    /// Byte data Gid value
    pub data: [u8; GID_STORAGE_SIZE],
}

pub trait GidTrait {
    /// Check if two unique identifiers (gids) are equal.
    fn compare_gids_equal(gid1: &Gid, gid2: &Gid, result: &mut bool) -> RetType;
}
