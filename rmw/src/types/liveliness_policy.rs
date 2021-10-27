// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

const SYSTEM_DEFAULT: &str = "system_default";
const AUTOMATIC: &str = "automatic";
const MANUAL_BY_TOPIC: &str = "manual_by_topic";

/// QoS liveliness enumerations that describe a publisher's reporting policy for its alive status.
/// For a subscriber, these are its requirements for its topic's publishers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QoSLivelinessPolicy {
    /// Implementation specific default
    SystemDefault = 0,

    /// The signal that establishes a Topic is alive comes from the R2 rmw layer.
    Automatic = 1,

    /// Explicitly asserting node liveliness is required in this case.
    #[deprecated(
        since = "0.1",
        note = "Use `ManualByTopic` if manually asserted liveliness is needed."
    )]
    ManualByNode = 2,

    /// The signal that establishes a Topic is alive is at the Topic level. Only publishing a message
    /// on the Topic or an explicit signal from the application to assert liveliness on the Topic
    /// will mark the Topic as being alive.
    ManualByTopic = 3,

    /// Liveliness policy has not yet been set
    Unknown = 4,
}

impl fmt::Display for QoSLivelinessPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SystemDefault => SYSTEM_DEFAULT,
            Self::Automatic => AUTOMATIC,
            Self::ManualByTopic => MANUAL_BY_TOPIC,
            _ => "",
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
        if s == MANUAL_BY_TOPIC {
            return Self::ManualByTopic;
        }
        Self::Unknown
    }
}
