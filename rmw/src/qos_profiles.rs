// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::qos_policy_kind::{
    QoSDurabilityPolicy, QoSHistoryPolicy, QoSLivelinessPolicy, QoSReliabilityPolicy,
};

/// R2 MiddleWare quality of service profile.
#[derive(Debug)]
pub struct QoSProfile {
    pub history: QoSHistoryPolicy,

    /// Size of the message queue.
    pub depth: usize,

    /// Reliabiilty QoS policy setting.
    pub reliability: QoSReliabilityPolicy,

    /// Durability QoS policy setting.
    pub durability: QoSDurabilityPolicy,

    /// The period at which messages are expected to be sent/received.
    //struct rmw_time_t deadline,
    pub deadline: u64,

    /// The age at which messages are considered expired and no longer valid.
    //struct rmw_time_t lifespan,
    pub lifespan: u64,

    /// Liveliness QoS policy setting
    pub liveliness: QoSLivelinessPolicy,

    /// The time within which the RMW node or publisher must show that it is alive.
    pub liveliness_lease_duration: u64,

    /// If true, any R2 specific namespacing conventions will be circumvented.
    pub avoid_ros_namespace_conventions: bool,
}
