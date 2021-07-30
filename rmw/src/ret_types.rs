// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Return code for rmw functions
pub type RetType = i32;

/// The operation ran as expected
pub const RMW_RET_OK: RetType = 0;
/// Generic error to indicate operation could not complete successfully
pub const RMW_RET_ERROR: RetType = 1;
/// The operation was halted early because it exceeded its timeout critera
pub const RMW_RET_TIMEOUT: RetType = 2;
/// The operation or event handling is not supported.
pub const RMW_RET_UNSUPPORTED: RetType = 3;

/// Failed to allocate memory
pub const RMW_RET_BAD_ALLOC: RetType = 10;
/// Argument to function was invalid
pub const RMW_RET_INVALID_ARGUMENT: RetType = 11;
/// Incorrect rmw implementation.
pub const RMW_RET_INCORRECT_RMW_IMPLEMENTATION: RetType = 12;

// rmw node specific ret codes in 2XX
/// Failed to find node name
// Using same return code than in rcl
pub const RMW_RET_NODE_NAME_NON_EXISTENT: RetType = 203;
