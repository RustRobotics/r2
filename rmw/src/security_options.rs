// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityEnforcementPolicy {
    Permissive = 0,
    Enforce,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityOptions {
    policy: SecurityEnforcementPolicy,
    security_root_path: String,
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

    pub fn security_root_path(&self) -> &str {
        &self.security_root_path
    }

    pub fn set_security_root_path(&mut self, security_root_path: &str) {
        self.security_root_path.clear();
        self.security_root_path.push_str(security_root_path);
    }
}

impl Default for SecurityOptions {
    fn default() -> Self {
        Self {
            policy: SecurityEnforcementPolicy::Permissive,
            security_root_path: String::new(),
        }
    }
}
