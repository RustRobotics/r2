// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

use super::context::Context;

/// Internal rcl guard condition implementation struct.
pub trait GuardConditionImpl: Debug {}

/// Handle for a rcl guard condition.
#[derive(Debug)]
pub struct GuardCondition {
    /// Context associated with this guard condition.
    pub context: Box<Context>,

    /// Pointer to the guard condition implementation
    pub imp: Box<dyn GuardConditionImpl>,
}

/// Options available for a rcl guard condition.
#[derive(Debug)]
pub struct GuardConditionOptions {}
