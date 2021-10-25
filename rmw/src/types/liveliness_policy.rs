// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

const SYSTEM_DEFAULT: &str = "system_default";
const AUTOMATIC: &str = "automatic";
const MANUAL_BY_NODE: &str = "manual_by_node";
const MANUAL_BY_TOPIC: &str = "manual_by_topic";

/// QoS liveliness enumerations that describe a publisher's reporting policy for its alive status.
/// For a subscriber, these are its requirements for its topic's publishers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSLivelinessPolicy {
    /// Implementation specific default
    SystemDefault = 0,

    /// The signal that establishes a Topic is alive comes from the R2 rmw layer.
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

impl fmt::Display for QoSLivelinessPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SystemDefault => SYSTEM_DEFAULT,
            Self::Automatic => AUTOMATIC,
            Self::ManualByNode => MANUAL_BY_NODE,
            Self::ManualByTopic => MANUAL_BY_TOPIC,
            Self::Unknown => "",
        };
        write!(f, "{}", s)
    }
}

impl QoSLivelinessPolicy {
    pub fn parse(s: &str) -> Self {
        if s == SYSTEM_DEFAULT {
            return Self::SystemDefault;
        }
        if s == AUTOMATIC {
            return Self::Automatic;
        }
        if s == MANUAL_BY_NODE {
            return Self::ManualByNode;
        }
        if s == MANUAL_BY_TOPIC {
            return Self::ManualByTopic;
        }
        Self::Unknown
    }
}
