// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::qos_policy_kind::QoSPolicyKind;

#[derive(Debug)]
pub struct QoSIncompatibleEventStatus {
    /// Total cumulative number of times the concerned subscription discovered a
    /// publisher for the same topic with an offered QoS that was incompatible
    /// with that requested by the subscription.
    pub total_count: i32,

    /// The change in total_count since the last time the status was read.
    pub total_count_change: i32,

    /// The Qos Policy Kind of one of the policies that was found to be
    /// incompatible the last time an incompatibility was detected.
    pub last_policy_kind: QoSPolicyKind,
}

/// Event state for a subscription's 'RMW_EVENT_REQUESTED_QOS_INCOMPATIBLE' events.
pub type RequestedQosIncompatibleEventStatus = QoSIncompatibleEventStatus;

/// Event state for a publisher's 'RMW_EVENT_OFFERED_QOS_INCOMPATIBLE' events.
pub type OfferedQosIncompatibleEventStatus = QoSIncompatibleEventStatus;
