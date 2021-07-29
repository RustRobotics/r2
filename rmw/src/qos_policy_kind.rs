// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSPolicyKind {
    Invalid = 0,
    Deadline,
    Liveliness,
    Reliability,
    History,
    Lifespan,
    Depth,
    LivelinessLeaseDuration,
    AvoidR2NamespaceConventions,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSReliabilityPolicy {
    /// Implementation specific default
    SystemDefault,

    /// Guarantee that samples are delivered, may retry multiple times.
    Reliable,

    /// Attempt to deliver samples, but some may be lost if the network is not robust
    BestEffort,

    /// Reliability policy has not yet been set
    Unknown,
}

/// QoS history enumerations describing how samples endure.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSHistoryPolicy {
    /// Implementation default for history policy
    SystemDefault = 0,

    /// Only store up to a maximum number of samples, dropping oldest once max is exceeded
    KeepLast,

    /// Store all samples, subject to resource limits
    KeepAll,

    /// History policy has not yet been set
    Unknown,
}

/// QoS durability enumerations describing how samples persist
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSDurabilityPolicy {
    /// Impplementation specific default
    SystemDefault = 0,

    /// The rmw publisher is responsible for persisting samples for “late-joining” subscribers
    TransientLocal,

    /// Samples are not persistent
    Volatile,

    /// Durability policy has not yet been set
    Unknown,
}

/// QoS liveliness enumerations that describe a publisher's reporting policy for its alive status.
/// For a subscriber, these are its requirements for its topic's publishers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSLivelinessPolicy {
    /// Implementation specific default
    SystemDefault = 0,

    /// The signal that establishes a Topic is alive comes from the ROS rmw layer.
    Automatic,

    /// Explicitly asserting node liveliness is required in this case.
    #[deprecated(since = "0.1", note = "Use `ManualByTopic` instead")]
    ManualByNode,

    /// The signal that establishes a Topic is alive is at the Topic level. Only publishing a message
    /// on the Topic or an explicit signal from the application to assert liveliness on the Topic
    /// will mark the Topic as being alive.
    ManualByTopic,

    /// Liveliness policy has not yet been set
    Unknown,
}
