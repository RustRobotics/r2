// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::types::MessageInfo;

/// Structure to hold a sequence of R2 messages.
#[derive(Debug, Default)]
pub struct MessageSequence {
    /// Array of pointers to R2 messages.
    //void ** data;
    pub data: Vec<usize>,
    // The number of valid entries in `data`.
    //pub size: usize,

    // The total allocated capacity of the data array.
    //pub capacity: usize,
}

impl MessageSequence {
    /// Return a MessageSequence struct with members initialized to `NULL`
    pub fn zero_initialized() -> Self {
        Self::default()
    }
}

/// Structure to hold a sequence of message infos.
#[derive(Debug, Default)]
pub struct MessageInfoSequence {
    /// Array of message info.
    pub data: Vec<MessageInfo>,
    // The number of valid entries in data.
    //pub size: usize,
    // The total allocated capacity of the data array.
    //pub capacity: usize,
}

impl MessageInfoSequence {
    /// Return a MessageInfoSequence struct with members initialized to `NULL`
    pub fn zero_initialized() -> Self {
        Self::default()
    }
}
