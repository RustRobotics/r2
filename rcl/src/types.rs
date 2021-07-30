// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rmw::ret_types;

pub type RetType = ret_types::RetType;

/// Success return code.
pub const RCL_RET_OK: RetType = ret_types::RMW_RET_OK;
/// Unspecified error return code.
pub const RCL_RET_ERROR: RetType = ret_types::RMW_RET_ERROR;
/// Timeout occurred return code.
pub const RCL_RET_TIMEOUT: RetType = ret_types::RMW_RET_TIMEOUT;
/// Failed to allocate memory return code.
pub const RCL_RET_BAD_ALLOC: RetType = ret_types::RMW_RET_BAD_ALLOC;
/// Invalid argument return code.
pub const RCL_RET_INVALID_ARGUMENT: RetType = ret_types::RMW_RET_INVALID_ARGUMENT;
/// Unsupported return code.
pub const RCL_RET_UNSUPPORTED: RetType = ret_types::RMW_RET_UNSUPPORTED;

// rcl specific ret codes start at 100
/// rcl_init() already called return code.
pub const RCL_RET_ALREADY_INIT: RetType = 100;
/// rcl_init() not yet called return code.
pub const RCL_RET_NOT_INIT: RetType = 101;
/// Mismatched rmw identifier return code.
pub const RCL_RET_MISMATCHED_RMW_ID: RetType = 102;
/// Topic name does not pass validation.
pub const RCL_RET_TOPIC_NAME_INVALID: RetType = 103;
/// Service name (same as topic name) does not pass validation.
pub const RCL_RET_SERVICE_NAME_INVALID: RetType = 104;
/// Topic name substitution is unknown.
pub const RCL_RET_UNKNOWN_SUBSTITUTION: RetType = 105;
/// rcl_shutdown() already called return code.
pub const RCL_RET_ALREADY_SHUTDOWN: RetType = 106;

// rcl node specific ret codes in 2XX
/// Invalid rcl_node_t given return code.
pub const RCL_RET_NODE_INVALID: RetType = 200;
/// Invalid node name return code.
pub const RCL_RET_NODE_INVALID_NAME: RetType = 201;

/// Invalid node namespace return code.
pub const RCL_RET_NODE_INVALID_NAMESPACE: RetType = 202;
/// Failed to find node name
pub const RCL_RET_NODE_NAME_NON_EXISTENT: RetType = 203;

// rcl publisher specific ret codes in 3XX
/// Invalid rcl_publisher_t given return code.
pub const RCL_RET_PUBLISHER_INVALID: RetType = 300;

// rcl subscription specific ret codes in 4XX
/// Invalid rcl_subscription_t given return code.
pub const RCL_RET_SUBSCRIPTION_INVALID: RetType = 400;
/// Failed to take a message from the subscription return code.
pub const RCL_RET_SUBSCRIPTION_TAKE_FAILED: RetType = 401;

// rcl service client specific ret codes in 5XX
/// Invalid rcl_client_t given return code.
pub const RCL_RET_CLIENT_INVALID: RetType = 500;
/// Failed to take a response from the client return code.
pub const RCL_RET_CLIENT_TAKE_FAILED: RetType = 501;

// rcl service server specific ret codes in 6XX
/// Invalid rcl_service_t given return code.
pub const RCL_RET_SERVICE_INVALID: RetType = 600;
/// Failed to take a request from the service return code.
pub const RCL_RET_SERVICE_TAKE_FAILED: RetType = 601;

// rcl guard condition specific ret codes in 7XX

// rcl timer specific ret codes in 8XX
/// Invalid rcl_timer_t given return code.
pub const RCL_RET_TIMER_INVALID: RetType = 800;
/// Given timer was canceled return code.
pub const RCL_RET_TIMER_CANCELED: RetType = 801;

// rcl wait and wait set specific ret codes in 9XX
/// Invalid rcl_wait_set_t given return code.
pub const RCL_RET_WAIT_SET_INVALID: RetType = 900;
/// Given rcl_wait_set_t is empty return code.
pub const RCL_RET_WAIT_SET_EMPTY: RetType = 901;
/// Given rcl_wait_set_t is full return code.
pub const RCL_RET_WAIT_SET_FULL: RetType = 902;

// rcl argument parsing specific ret codes in 1XXX
/// Argument is not a valid remap rule
pub const RCL_RET_INVALID_REMAP_RULE: RetType = 1001;
/// Expected one type of lexeme but got another
pub const RCL_RET_WRONG_LEXEME: RetType = 1002;
/// Found invalid r2 argument while parsing
pub const RCL_RET_INVALID_R2_ARGS: RetType = 1003;
/// Argument is not a valid parameter rule
pub const RCL_RET_INVALID_PARAM_RULE: RetType = 1010;
/// Argument is not a valid log level rule
pub const RCL_RET_INVALID_LOG_LEVEL_RULE: RetType = 1020;

// rcl event specific ret codes in 20XX
/// Invalid rcl_event_t given return code.
pub const RCL_RET_EVENT_INVALID: RetType = 2000;
/// Failed to take an event from the event handle
pub const RCL_RET_EVENT_TAKE_FAILED: RetType = 2001;

/// rcl_lifecycle state register ret codes in 30XX
/// rcl_lifecycle state registered
pub const RCL_RET_LIFECYCLE_STATE_REGISTERED: RetType = 3000;
/// rcl_lifecycle state not registered
pub const RCL_RET_LIFECYCLE_STATE_NOT_REGISTERED: RetType = 3001;

/// typedef for rmw_serialized_message_t;
pub type SerializedMessage = rmw::serialized_message::SerializedMessage;
