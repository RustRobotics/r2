// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

const SYSTEM_DEFAULT: &str = "system_default";
const KEEP_LAST: &str = "keep_last";
const KEEP_ALL: &str = "keep_all";

/// QoS history enumerations describing how samples endure.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl fmt::Display for QoSHistoryPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SystemDefault => SYSTEM_DEFAULT,
            Self::KeepLast => KEEP_LAST,
            Self::KeepAll => KEEP_ALL,
            Self::Unknown => "",
        };
        write!(f, "{}", s)
    }
}

impl QoSHistoryPolicy {
    pub fn parse(s: &str) -> Self {
        if s == SYSTEM_DEFAULT {
            return Self::SystemDefault;
        }
        if s == KEEP_LAST {
            return Self::KeepLast;
        }
        if s == KEEP_ALL {
            return Self::KeepAll;
        }
        Self::Unknown
    }
}
