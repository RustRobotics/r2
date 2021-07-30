// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

// Re-export types.
pub use r2utils::time::TimePointValue;

/// Time source type, used to indicate the source of a time measurement.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClockType {
    /// Clock uninitialized
    Uninitialized = 0,

    /// Use R2 time
    ///
    /// R2Time will report the latest value reported by a R2 time source, or
    /// if a R2 time source is not active it reports the same as SystemTime.
    R2Time,

    /// Use system time
    ///
    /// SystemTime reports the same value as the system clock.
    SystemTime,

    /// Use a steady clock time.
    ///
    /// SteadyTime reports a value from a monotonically increasing clock.
    SteadyTime,
}

/// Enumeration to describe the type of time jump.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClockChange {
    /// The source before and after the jump is R2Time.
    R2TimeNoChange = 1,

    /// The source switched to R2_TIME from SystemTime.
    R2TimeActivated = 2,

    /// The source switched to SYSTEM_TIME from R2Time.
    R2TimeDeactivated = 3,

    /// The source before and after the jump is SystemTime.
    SystemTimeNoChange = 4,
}

/// Struct to describe a jump in time.
#[derive(Debug)]
pub struct TimeJump {
    /// Indicate whether or not the source of time changed.
    pub clock_change: ClockChange,

    /// The new time minus the last time before the jump.
    pub delta: Duration,
}

/// Describe the prerequisites for calling a time jump callback.
#[derive(Debug)]
pub struct JumpThreshold {
    /// True to call callback when the clock type changes.
    pub on_clock_change: bool,

    /// A positive duration indicating the minimum jump forwards to be considered exceeded, or zero
    /// to disable.
    pub min_forward: Duration,

    /// A negative duration indicating the minimum jump backwards to be considered exceeded, or zero
    /// to disable.
    pub min_backward: Duration,
}

/// Struct to describe an added callback.
#[derive(Debug)]
pub struct JumpCallbackInfo {
    /// Callback to fucntion.
    //rcl_jump_callback_t callback;

    /// Threshold to decide when to call the callback.
    pub threshold: JumpThreshold,

    /// Pointer passed to the callback.
    //void * user_data;
    pub user_data: *const u8,
}

/// Encapsulation of a time source.
#[derive(Debug)]
pub struct Clock {
    /// Clock type
    pub type_: ClockType,

    /// An array of added jump callbacks.
    //rcl_jump_callback_info_t * jump_callbacks;

    /// Number of callbacks in jump_callbacks.
    pub num_jump_callbacks: usize,

    /// Pointer to get_now function
    //rcl_ret_t (* get_now)(void * data, rcl_time_point_value_t * now);
    // void (*set_now) (rcl_time_point_value_t);

    /// Clock storage
    //void * data;
    pub data: *const u8,
}

/// A single point in time, measured in nanoseconds, the reference point is based on the source.
#[derive(Debug)]
pub struct TimePoint {
    /// Nanoseconds of the point in time
    pub nanoseconds: TimePointValue,

    /// Clock type of the point in time
    pub clock_type: ClockType,
}
