// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub const NODE_NAME_MAX_NAME_LENGTH: usize = 255;

#[derive(Debug)]
pub enum NodeNameErrorType {
    /// Node name is empty.
    EmptyString,

    /// Only alphanumeric characters and underscores are allowed.
    ContainsUnallowedChars,

    /// Must not start with a number.
    StartWithNumber,

    /// Length of node name is larger than 255.
    TooLong,
}

#[derive(Debug)]
pub struct NodeNameError {
    pub reason: NodeNameErrorType,
    pub invalid_index: usize,
}

impl NodeNameError {
    pub fn new(reason: NodeNameErrorType, invalid_index: usize) -> Self {
        Self {
            reason,
            invalid_index,
        }
    }
}

/// Determine if a node name is valid.
/// Node names must follow these rules:
///   - must not be an empty string
///   - must only contain alphanumeric characters and underscores (a-z|A-Z|0-9|_)
///   - must not start with a number
pub fn validate_node_name(name: &str) -> Result<(), NodeNameError> {
    if (name.is_empty()) {
        return Err(NodeNameError::new(NodeNameErrorType::EmptyString, 0));
    }
    if name.len() > NODE_NAME_MAX_NAME_LENGTH {
        return Err(NodeNameError::new(
            NodeNameErrorType::TooLong,
            NODE_NAME_MAX_NAME_LENGTH + 1,
        ));
    }

    // Check for allowed characters.
    if let Some(first_char) = name.chars().next() {
        if first_char.is_digit(10) {
            return Err(NodeNameError::new(NodeNameErrorType::StartWithNumber, 0));
        }
    }

    for (idx, c) in name.chars().enumerate() {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(NodeNameError::new(
                NodeNameErrorType::ContainsUnallowedChars,
                idx,
            ));
        }
    }

    Ok(())
}
