// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ffi::{OsStr, OsString};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityEnforcementPolicy {
    Permissive,
    Enforce,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityOptions {
    policy: SecurityEnforcementPolicy,
    security_root_path: OsString,
}

impl SecurityOptions {
    /// Get zero initialized security options.
    #[inline]
    pub fn zero_initialized() -> Self {
        Self::default()
    }

    pub fn policy(&self) -> SecurityEnforcementPolicy {
        self.policy
    }

    pub fn security_root_path(&self) -> &OsStr {
        &self.security_root_path
    }

    pub fn set_security_root_path<T: AsRef<OsStr>>(&mut self, security_root_path: T) {
        self.security_root_path.clear();
        self.security_root_path.push(security_root_path.as_ref());
    }
}

impl Default for SecurityOptions {
    fn default() -> Self {
        Self {
            policy: SecurityEnforcementPolicy::Permissive,
            security_root_path: OsString::new(),
        }
    }
}
