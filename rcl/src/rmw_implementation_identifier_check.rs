// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// The environment variable name to control which RMW implementation is used.
pub const RMW_IMPLEMENTATION_ENV_VAR_NAME: &str = "RMW_IMPLEMENTATION";

/// The environment variable name to control whether the chosen RMW implementation
/// matches the one that is in use.
pub const RCL_ASSERT_RMW_ID_MATCHES_ENV_VAR_NAME: &str = "RCL_ASSERT_RMW_ID_MATCHES";
