// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub struct MessageLostStatus {
    /// Total number of messages lost.
    pub total_count: usize,

    /// Number of messages lost since last callback.
    pub total_count_change: usize,
}
