// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

const SYSTEM_DEFAULT: &str = "system_default";
const TRANSIENT_LOCAL: &str = "transient_local";
const VOLATILE: &str = "volatile";

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

impl fmt::Display for QoSDurabilityPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SystemDefault => SYSTEM_DEFAULT,
            Self::TransientLocal => TRANSIENT_LOCAL,
            Self::Volatile => VOLATILE,
            Self::Unknown => "",
        };
        write!(f, "{}", s)
    }
}

impl QoSDurabilityPolicy {
    pub fn from_str(s: &str) -> Self {
        if s == SYSTEM_DEFAULT {
            return Self::SystemDefault;
        }
        if s == TRANSIENT_LOCAL {
            return Self::TransientLocal;
        }
        if s == VOLATILE {
            return Self::Volatile;
        }
        Self::Unknown
    }
}
