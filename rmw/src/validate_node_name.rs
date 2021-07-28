// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Length of node name must not exceed 255 characters.
pub const NODE_NAME_MAX_NAME_LENGTH: usize = 255;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum NodeNameErrorType {
    /// Node name is empty.
    EmptyString,

    /// Only alphanumeric characters and underscores are allowed.
    ContainsUnallowedChars,

    /// Must not start with a number.
    StartsWithNumber,

    /// Length of node name is larger than 255.
    TooLong,
}

#[derive(Debug, Clone, Copy)]
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
///
/// The `NodeNameErrorType::TooLong` is guaranteed to be checked last, such
/// that if you get that result, then you can assume all other checks succeeded.
/// This is done so that the length limit can be treated as a warning rather
/// than an error if desired.
///
/// Node names must follow these rules:
///   - must not be an empty string
///   - must only contain alphanumeric characters and underscores (a-z|A-Z|0-9|_)
///   - must not start with a number
pub fn validate_node_name(node_name: &str) -> Result<(), NodeNameError> {
    if node_name.is_empty() {
        return Err(NodeNameError::new(NodeNameErrorType::EmptyString, 0));
    }

    // Check for allowed characters.
    for (idx, c) in node_name.chars().enumerate() {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(NodeNameError::new(
                NodeNameErrorType::ContainsUnallowedChars,
                idx,
            ));
        }
    }

    // This is the case where the name starts with a number, i.e. [0-9].
    if node_name.starts_with(|c: char| c.is_digit(10)) {
        return Err(NodeNameError::new(NodeNameErrorType::StartsWithNumber, 0));
    }

    // Check if the node name is too long last, since it might be a soft invalidation
    if node_name.len() > NODE_NAME_MAX_NAME_LENGTH {
        return Err(NodeNameError::new(
            NodeNameErrorType::TooLong,
            NODE_NAME_MAX_NAME_LENGTH - 1,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_node_name() {
        assert!(validate_node_name("nodename").is_ok());
    }

    #[test]
    fn test_empty_node_name() {
        let ret = validate_node_name("");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::EmptyString);
        assert_eq!(err.invalid_index, 0);
    }

    #[test]
    fn test_unallowed_chars() {
        let ret = validate_node_name("node/name");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::ContainsUnallowedChars);
        assert_eq!(err.invalid_index, 4);

        let ret = validate_node_name("node_{name}");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::ContainsUnallowedChars);
        assert_eq!(err.invalid_index, 5);

        let ret = validate_node_name("~node_name");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::ContainsUnallowedChars);
        assert_eq!(err.invalid_index, 0);

        let ret = validate_node_name("with spaces");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::ContainsUnallowedChars);
        assert_eq!(err.invalid_index, 4);

        let ret = validate_node_name("with.periods");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::ContainsUnallowedChars);
        assert_eq!(err.invalid_index, 4);
    }

    #[test]
    fn test_starts_with_number() {
        let ret = validate_node_name("42node");
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::StartsWithNumber);
        assert_eq!(err.invalid_index, 0);
    }

    #[test]
    fn test_node_name_too_long() {
        // Ensure the length is not the first error
        let long_name: String = "0".repeat(NODE_NAME_MAX_NAME_LENGTH + 1);
        let ret = validate_node_name(&long_name);
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::StartsWithNumber);
        assert_eq!(err.invalid_index, 0);

        // Ensure length check works when there are no other issues
        let long_name: String = "a".repeat(NODE_NAME_MAX_NAME_LENGTH + 1);
        let ret = validate_node_name(&long_name);
        assert!(ret.is_err());
        let err = ret.err().unwrap();
        assert_eq!(err.reason, NodeNameErrorType::TooLong);
        assert_eq!(err.invalid_index, NODE_NAME_MAX_NAME_LENGTH - 1);
    }
}
