// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

/// Constant representing an infinite duration.
#[inline]
pub fn duration_infinite() -> Duration {
    Duration::new(9223372036, 854775807)
}

#[inline]
pub fn duration_unspecified() -> Duration {
    Duration::new(0, 0)
}
