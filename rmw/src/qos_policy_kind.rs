// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

const DURABILITY: &str = "durability";
const DEADLINE: &str = "deadline";
const LIVELINESS: &str = "liveliness";
const RELIABILITY: &str = "reliability";
const HISTORY: &str = "history";
const LIFESPAN: &str = "lifespan";
const DEPTH: &str = "depth";
const LIVELINESS_LEASE_DURATION: &str = "liveliness_lease_duration";
const AVOID_R2_NAMESPACE_CONVENTIONS: &str = "avoid_r2_namespace_conventions";

/// QOS policy kind.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum QoSPolicyKind {
    //Invalid = 0,
    Durability = 1,
    Deadline,
    Liveliness,
    Reliability,
    History,
    Lifespan,
    Depth,
    LivelinessLeaseDuration,
    AvoidR2NamespaceConventions,
}

/// Return a string representing the policy kind.
impl fmt::Display for QoSPolicyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Durability => DURABILITY,
            Self::Deadline => DEADLINE,
            Self::Liveliness => LIVELINESS,
            Self::Reliability => RELIABILITY,
            Self::History => HISTORY,
            Self::Lifespan => LIFESPAN,
            Self::Depth => DEPTH,
            Self::LivelinessLeaseDuration => LIVELINESS_LEASE_DURATION,
            Self::AvoidR2NamespaceConventions => AVOID_R2_NAMESPACE_CONVENTIONS,
        };
        write!(f, "{}", s)
    }
}

pub struct ParseQoSPlicyKindError {
    pub reason: String,
}

impl std::str::FromStr for QoSPolicyKind {
    type Err = ParseQoSPlicyKindError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == DURABILITY {
            return Ok(Self::Durability);
        }
        if s == DEADLINE {
            return Ok(Self::Deadline);
        }
        if s == LIVELINESS {
            return Ok(Self::Liveliness);
        }
        if s == RELIABILITY {
            return Ok(Self::Reliability);
        }
        if s == HISTORY {
            return Ok(Self::History);
        }
        if s == LIFESPAN {
            return Ok(Self::Lifespan);
        }
        if s == DEPTH {
            return Ok(Self::Depth);
        }
        if s == LIVELINESS_LEASE_DURATION {
            return Ok(Self::LivelinessLeaseDuration);
        }
        if s == AVOID_R2_NAMESPACE_CONVENTIONS {
            return Ok(Self::AvoidR2NamespaceConventions);
        }
        Err(ParseQoSPlicyKindError {
            reason: format!("Invalid qos policy kind in {}", s),
        })
    }
}
