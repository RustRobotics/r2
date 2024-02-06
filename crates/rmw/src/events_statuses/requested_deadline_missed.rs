// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// QoS Requested Deadline Missed information provided by a subscription.
#[derive(Debug)]
pub struct RequestedDeadlineMissedStatus {
    /// Lifetime cumulative number of missed deadlines detected for any instance read by the
    /// subscription.
    /// Missed deadlines accumulate; that is, each deadline period the total_count will be incremented
    /// by one for each instance for which data was not received.
    pub total_count: i32,

    /// The incremental number of deadlines detected since the status was read.
    pub total_count_change: i32,
}
