// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::types::MessageInfo;

/// Structure to hold a sequence of ROS messages.
#[derive(Debug)]
pub struct MessageSequence {
    /// Array of pointers to R2 messages.
    //void ** data;
    pub data: *const u8,

    /// The number of valid entries in `data`.
    pub size: usize,

    /// The total allocated capacity of the data array.
    pub capacity: usize,
}

/// Structure to hold a sequence of message infos.
#[derive(Debug)]
pub struct MessageInfoSequence {
    /// Array of message info.
    pub data: *const MessageInfo,

    /// The number of valid entries in data.
    pub size: usize,

    /// The total allocated capacity of the data array.
    pub capacity: usize,
}
