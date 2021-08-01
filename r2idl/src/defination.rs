// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Basic types as defined by the IDL specification

// 7.4.1.4.4.2 Basic Types
// rules (26)
pub const SIGNED_NONEXPLICIT_INTEGER_TYPES: [&str; 3] = [
    "short",     // rule (27)
    "long",      // rule (28)
    "long long", // rule (29)
];

// rules (30)
pub const UNSIGNED_NONEXPLICIT_INTEGER_TYPES: [&str; 3] = [
    "unsigned short",     // rule (31)
    "unsigned long",      // rule (32)
    "unsigned long long", // rule (33)
];

pub const NONEXPLICIT_INTEGER_TYPES: [&str; 6] = [
    //SIGNED_NONEXPLICIT_INTEGER_TYPES,
    "short",     // rule (27)
    "long",      // rule (28)
    "long long", // rule (29)
    //UNSIGNED_NONEXPLICIT_INTEGER_TYPES,
    "unsigned short",     // rule (31)
    "unsigned long",      // rule (32)
    "unsigned long long", // rule (33)
];

// rule (24)
pub const FLOATING_POINT_TYPES: [&str; 3] = ["float", "double", "long double"];

pub const CHARACTER_TYPES: [&str; 2] = [
    "char",  // rule (34)
    "wchar", // rule (35)
];

pub const BOOLEAN_TYPE: &str = "boolean"; // rule (36)
pub const OCTET_TYPE: &str = "octet"; // rule (37)

// 7.4.13.4.4 Integers restricted to holding 8-bits of information
// 7.4.13.4.5 Explicitly-named Integer Types
pub const SIGNED_EXPLICIT_INTEGER_TYPES: [&str; 4] = [
    "int8",  // rule (208)
    "int16", // rule (210)
    "int32", // rule (211)
    "int64", // rule (212)
];

pub const UNSIGNED_EXPLICIT_INTEGER_TYPES: [&str; 4] = [
    "uint8",  // rule (209)
    "uint16", // rule (213)
    "uint32", // rule (214)
    "uint64", // rule (215)
];

pub const EXPLICIT_INTEGER_TYPES: [&str; 8] = [
    //SIGNED_EXPLICIT_INTEGER_TYPES,
    "int8",  // rule (208)
    "int16", // rule (210)
    "int32", // rule (211)
    "int64", // rule (212)
    //UNSIGNED_EXPLICIT_INTEGER_TYPES,
    "uint8",  // rule (209)
    "uint16", // rule (213)
    "uint32", // rule (214)
    "uint64", // rule (215)
];

// rules (26) + (208) + (210-212)
pub const SIGNED_INTEGER_TYPES: [&str; 7] = [
    //SIGNED_NONEXPLICIT_INTEGER_TYPES,
    "short",     // rule (27)
    "long",      // rule (28)
    "long long", // rule (29)
    //SIGNED_EXPLICIT_INTEGER_TYPES,
    "int8",  // rule (208)
    "int16", // rule (210)
    "int32", // rule (211)
    "int64", // rule (212)
];

// rules (30) + (209) + (213-215)
pub const UNSIGNED_INTEGER_TYPES: [&str; 7] = [
    //UNSIGNED_NONEXPLICIT_INTEGER_TYPES[..],
    "unsigned short",     // rule (31)
    "unsigned long",      // rule (32)
    "unsigned long long", // rule (33)
    //UNSIGNED_EXPLICIT_INTEGER_TYPES[..],
    "uint8",  // rule (209)
    "uint16", // rule (213)
    "uint32", // rule (214)
    "uint64", // rule (215)
];

// rules (25) + (206-207) + (210-215)
pub const INTEGER_TYPES: [&str; 14] = [
    //SIGNED_INTEGER_TYPES,
    //SIGNED_NONEXPLICIT_INTEGER_TYPES,
    "short",     // rule (27)
    "long",      // rule (28)
    "long long", // rule (29)
    //SIGNED_EXPLICIT_INTEGER_TYPES,
    "int8",  // rule (208)
    "int16", // rule (210)
    "int32", // rule (211)
    "int64", // rule (212)
    //UNSIGNED_INTEGER_TYPES
    //UNSIGNED_NONEXPLICIT_INTEGER_TYPES[..],
    "unsigned short",     // rule (31)
    "unsigned long",      // rule (32)
    "unsigned long long", // rule (33)
    //UNSIGNED_EXPLICIT_INTEGER_TYPES[..],
    "uint8",  // rule (209)
    "uint16", // rule (213)
    "uint32", // rule (214)
    "uint64", // rule (215)
];

// All basic types as defined by the IDL specification.
pub const BASIC_TYPES: [&str; 21] = [
    //INTEGER_TYPES,
    //SIGNED_INTEGER_TYPES,
    //SIGNED_NONEXPLICIT_INTEGER_TYPES,
    "short",     // rule (27)
    "long",      // rule (28)
    "long long", // rule (29)
    //SIGNED_EXPLICIT_INTEGER_TYPES,
    "int8",  // rule (208)
    "int16", // rule (210)
    "int32", // rule (211)
    "int64", // rule (212)
    //UNSIGNED_INTEGER_TYPES
    //UNSIGNED_NONEXPLICIT_INTEGER_TYPES[..],
    "unsigned short",     // rule (31)
    "unsigned long",      // rule (32)
    "unsigned long long", // rule (33)
    //UNSIGNED_EXPLICIT_INTEGER_TYPES[..],
    "uint8",  // rule (209)
    "uint16", // rule (213)
    "uint32", // rule (214)
    "uint64", // rule (215)
    //FLOATING_POINT_TYPES,
    "float",
    "double",
    "long double",
    //CHARACTER_TYPES,
    "char",  // rule (34)
    "wchar", // rule (35)
    //BOOLEAN_TYPE,
    "boolean",
    //OCTET_TYPE,
    "octet",
];

pub const EMPTY_STRUCTURE_REQUIRED_MEMBER_NAME: &str = "structure_needs_at_least_one_member";

pub const CONSTANT_MODULE_SUFFIX: &str = "_Constants";

pub const SERVICE_REQUEST_MESSAGE_SUFFIX: &str = "_Request";
pub const SERVICE_RESPONSE_MESSAGE_SUFFIX: &str = "_Response";

pub const ACTION_GOAL_SUFFIX: &str = "_Goal";
pub const ACTION_RESULT_SUFFIX: &str = "_Result";
pub const ACTION_FEEDBACK_SUFFIX: &str = "_Feedback";

pub const ACTION_GOAL_SERVICE_SUFFIX: &str = "_SendGoal";
pub const ACTION_RESULT_SERVICE_SUFFIX: &str = "_GetResult";
pub const ACTION_FEEDBACK_MESSAGE_SUFFIX: &str = "_FeedbackMessage";
