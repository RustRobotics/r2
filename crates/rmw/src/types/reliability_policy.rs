// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

const SYSTEM_DEFAULT: &str = "system_default";
const RELIABLE: &str = "reliable";
const BEST_EFFORT: &str = "best_effort";

#[repr(u8)]
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

impl fmt::Display for QoSReliabilityPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SystemDefault => SYSTEM_DEFAULT,
            Self::Reliable => RELIABLE,
            Self::BestEffort => BEST_EFFORT,
            Self::Unknown => "",
        };
        write!(f, "{}", s)
    }
}

impl QoSReliabilityPolicy {
    pub fn parse(s: &str) -> Self {
        if s == SYSTEM_DEFAULT {
            return Self::SystemDefault;
        }
        if s == RELIABLE {
            return Self::Reliable;
        }
        if s == BEST_EFFORT {
            return Self::BestEffort;
        }
        Self::Unknown
    }
}
