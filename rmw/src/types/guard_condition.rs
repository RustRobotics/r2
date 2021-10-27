// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::init::Context;
use crate::ret_types::RetType;

/// Handle for an rmw guard condition
#[derive(Debug)]
pub struct GuardCondition {
    /// The name of the rmw implementation
    pub implementation_identifier: String,

    /// Type erased pointer to this guard condition
    //void * data;
    pub data: *const u8,

    /// rmw context associated with this guard condition
    pub context: Box<Context>,
}

/// Array of guard condition handles.
///
/// An array of void * pointers representing type-erased middleware-specific guard conditions.
/// The number of non-null entries may be smaller than the allocated size of the array.
/// The number of guard conditions represented may be smaller than the allocated size of the array.
/// The creator of this struct is responsible for allocating and deallocating the array.
#[derive(Debug)]
pub struct GuardConditions(Vec<GuardCondition>);

pub trait GuardConditionTrait {
    fn trigger(guard_condition: &GuardCondition) -> RetType;
}
