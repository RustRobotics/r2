// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

/// Internal rcl action goal implementation struct.
pub trait GoalHandleImpl: Debug {}

/// Goal handle for an action.
#[derive(Debug)]
pub struct GoalHandle {
    /// Pointer to the action goal handle implementation
    pub imp: Box<dyn GoalHandleImpl>,
}
