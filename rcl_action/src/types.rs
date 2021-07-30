// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub use rmw::ret_types::RetType;

// rcl action specific ret codes in 2XXX
/// Action name does not pass validation return code.
pub const RCL_RET_ACTION_NAME_INVALID: RetType = 2000;
/// Action goal accepted return code.
pub const RCL_RET_ACTION_GOAL_ACCEPTED: RetType = 2100;
/// Action goal rejected return code.
pub const RCL_RET_ACTION_GOAL_REJECTED: RetType = 2101;
/// Action client is invalid return code.
pub const RCL_RET_ACTION_CLIENT_INVALID: RetType = 2102;
/// Action client failed to take response return code.
pub const RCL_RET_ACTION_CLIENT_TAKE_FAILED: RetType = 2103;
/// Action server is invalid return code.
pub const RCL_RET_ACTION_SERVER_INVALID: RetType = 2200;
/// Action server failed to take request return code.
pub const RCL_RET_ACTION_SERVER_TAKE_FAILED: RetType = 2201;
/// Action goal handle invalid return code.
pub const RCL_RET_ACTION_GOAL_HANDLE_INVALID: RetType = 2300;
/// Action invalid event return code.
pub const RCL_RET_ACTION_GOAL_EVENT_INVALID: RetType = 2301;

pub const UUID_SIZE: usize = 16;

/// Struct with the action goal status array
#[derive(Debug)]
pub struct ActionGoalStatusArray {
    // Goal status array message
//action_msgs__msg__GoalStatusArray msg;
}

/// Struct with the action cancel response
#[derive(Debug)]
pub struct ActionCancelResponse {
    // Cancel goal response message
//action_msgs__srv__CancelGoal_Response msg;
}

/// Goal state transition events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionGoalEvent {
    Execute = 0,
    CancelGoal,
    Succeed,
    Abort,
    Canceled,
    NumEvents,
}
