// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

use crate::ret_types::RetType;
use crate::time::duration_unspecified;
use crate::types::{
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
    pub avoid_r2_namespace_conventions: bool,
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
            avoid_r2_namespace_conventions: false,
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
            avoid_r2_namespace_conventions: false,
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
            avoid_r2_namespace_conventions: false,
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
            avoid_r2_namespace_conventions: false,
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
            avoid_r2_namespace_conventions: false,
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
            avoid_r2_namespace_conventions: false,
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
            avoid_r2_namespace_conventions: false,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QoSCompatibilityType {
    /// QoS policies are compatible
    Ok = 0,

    /// QoS policies may not be compatible
    Warning,

    /// QoS policies are not compatible
    Error,
}

pub trait QoSProfileTrait {
    /// Check if two QoS profiles are compatible.
    ///
    /// Two QoS profiles are compatible if a publisher and subcription
    /// using the QoS policies can communicate with each other.
    ///
    /// If any of the profile policies has the value "system default" or "unknown", then it may not be
    /// possible to determine the compatibilty.
    /// In this case, the output parameter `compatibility` is set to `Warning`
    /// and `reason` is populated.
    ///
    /// If there is a compatibility warning or error, and a buffer is provided for `reason`, then an
    /// explanation of all warnings and errors will be populated into the buffer, separated by semi-colons (`;`).
    /// Errors will appear before warnings in the string buffer.
    /// If the provided buffer is not large enough, this function will still write to the buffer, up to
    /// the `reason_size` number of characters.
    /// Therefore, it is possible that not all errors and warnings are communicated if the buffer size limit is reached.
    /// A buffer size of 2048 should be more than enough to capture all possible errors and warnings.
    ///
    /// Return `RET_OK` if the check was successful,
    /// or return `RET_ERROR` if there is an unexpected error.
    fn check_compatible(
        &self,
        other: &Self,
        compatibility: &mut QoSCompatibilityType,
        reason: &mut String,
    ) -> RetType;
}
