// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// QoS Liveliness Changed information provided by a subscription.
#[derive(Debug)]
pub struct LivelinessChangedStatus {
    /// The total number of currently active Publishers which publish to the topic associated with
    /// the Subscription.
    /// This count increases when a newly matched Publisher asserts its liveliness for the first time
    /// or when a Publisher previously considered to be not alive reasserts its liveliness.
    /// The count decreases when a Publisher considered alive fails to assert its liveliness and
    /// becomes not alive, whether because it was deleted normally or for some other reason.
    ///
    pub alive_count: i32,

    /// The total count of current Publishers which publish to the topic associated with the
    /// Subscription that are no longer asserting their liveliness.
    /// This count increases when a Publisher considered alive fails to assert its liveliness and
    /// becomes not alive for some reason other than the normal deletion of that Publisher.
    /// It decreases when a previously not alive Publisher either reasserts its liveliness or is
    /// deleted normally.
    pub not_alive_count: i32,

    /// The change in the alive_count since the status was last read.
    pub alive_count_change: i32,

    /// The change in the not_alive_count since the status was last read.
    pub not_alive_count_change: i32,
}
