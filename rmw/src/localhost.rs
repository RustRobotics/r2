// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Used to specify if the context can only communicate through localhost.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocalhostOnly {
    /// Uses `R2_LOCALHOST_ONLY` environment variable.
    UseDefault = 0,

    /// Forces using only localhost.
    Enabled = 1,

    /// Forces disabling localhost only.
    Disabled = 2,
}
