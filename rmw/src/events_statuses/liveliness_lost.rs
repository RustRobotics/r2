// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// QoS Liveliness Lost information provided by a publisher.
#[derive(Debug)]
pub struct LivelinessLostStatus {
    /// Lifetime cumulative number of times that a previously-alive Publisher became not alive due to
    /// a failure to actively signal its liveliness within its offered liveliness period.
    /// This count does not change when an already not alive Publisher simply remains not alive for
    /// another liveliness period.
    pub total_count: i32,

    /// The change in total_count since the last time the status was last read.
    pub total_count_change: i32,
}
