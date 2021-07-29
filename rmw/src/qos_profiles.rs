// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

use super::time::duration_unspecified;
use super::types::{
    QoSDurabilityPolicy, QoSHistoryPolicy, QoSLivelinessPolicy, QoSReliabilityPolicy,
};

pub const QOS_POLICY_DEPTH_SYSTEM_DEFAULT: usize = 0;

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
    pub deadline: Duration,

    /// The age at which messages are considered expired and no longer valid.
    pub lifespan: Duration,

    /// Liveliness QoS policy setting
    pub liveliness: QoSLivelinessPolicy,

    /// The time within which the RMW node or publisher must show that it is alive.
    pub liveliness_lease_duration: Duration,

    /// If true, any R2 specific namespacing conventions will be circumvented.
    pub avoid_ros_namespace_conventions: bool,
}

impl QoSProfile {
    #[inline]
    pub fn sensor_data() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::KeepLast,
            depth: 5,
            reliability: QoSReliabilityPolicy::BestEffort,
            durability: QoSDurabilityPolicy::Volatile,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::SystemDefault,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }

    #[inline]
    pub fn parameters() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::KeepLast,
            depth: 1000,
            reliability: QoSReliabilityPolicy::Reliable,
            durability: QoSDurabilityPolicy::Volatile,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::SystemDefault,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }

    #[inline]
    pub fn services_default() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::KeepLast,
            depth: 10,
            reliability: QoSReliabilityPolicy::Reliable,
            durability: QoSDurabilityPolicy::Volatile,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::SystemDefault,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }

    #[inline]
    pub fn parameter_events() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::KeepLast,
            depth: 1000,
            reliability: QoSReliabilityPolicy::Reliable,
            durability: QoSDurabilityPolicy::Volatile,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::SystemDefault,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }

    #[inline]
    pub fn system_default() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::SystemDefault,
            depth: QOS_POLICY_DEPTH_SYSTEM_DEFAULT,
            reliability: QoSReliabilityPolicy::SystemDefault,
            durability: QoSDurabilityPolicy::SystemDefault,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::SystemDefault,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }

    #[inline]
    pub fn unknown() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::Unknown,
            depth: QOS_POLICY_DEPTH_SYSTEM_DEFAULT,
            reliability: QoSReliabilityPolicy::Unknown,
            durability: QoSDurabilityPolicy::Unknown,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::Unknown,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }
}

impl Default for QoSProfile {
    fn default() -> Self {
        QoSProfile {
            history: QoSHistoryPolicy::KeepLast,
            depth: 10,
            reliability: QoSReliabilityPolicy::Reliable,
            durability: QoSDurabilityPolicy::Volatile,
            deadline: duration_unspecified(),
            lifespan: duration_unspecified(),
            liveliness: QoSLivelinessPolicy::SystemDefault,
            liveliness_lease_duration: duration_unspecified(),
            avoid_ros_namespace_conventions: false,
        }
    }
}

// TODO(Shaohua): Add compatibility type.
