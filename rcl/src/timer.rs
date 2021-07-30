// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

pub trait TimerImpl: Debug {}

/// Structure which encapsulates a ROS Timer.
#[derive(Debug)]
pub struct Timer {
    /// Private implementation pointer.
    pub imp: Box<dyn TimerImpl>,
}
