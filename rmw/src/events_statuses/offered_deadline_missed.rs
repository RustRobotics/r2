// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// QoS Deadline Missed information provided by a publisher.
#[derive(Debug)]
pub struct OfferedDeadlineMissedStatus {
    /// Lifetime cumulative number of offered deadline periods elapsed during which a Publisher failed
    /// to provide data.
    /// Missed deadlines accumulate; that is, each deadline period the total_count will be incremented
    /// by one.
    pub total_count: i32,

    /// The change in total_count since the last time the status was last read.
    pub total_count_change: i32,
}
