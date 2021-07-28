// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::validate_topic_name::{
    validate_topic_name, TopicNameError, TopicNameErrorType, TOPIC_NAME_MAX_LENGTH,
};

/// An additional 2 characters are reserved for the shortest possible topic, e.g. `/X`.
pub const NAMESPACE_MAX_LENGTH: usize = TOPIC_NAME_MAX_LENGTH - 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamespaceErrorType {
    /// Node name is empty.
    EmptyString,

    /// Not starts with `/`.
    NotAbsolute,

    /// Must not ends with `/`.
    EndsWithForwardSlash,

    /// Only alphanumeric characters and underscores are allowed.
    ContainsUnallowedChars,

    /// A slash must not be followed by another slash.
    ContainsRepeatedForwardSlash,

    /// Must not start with a number.
    StartsWithNumber,

    /// Length of node name is larger than 255.
    TooLong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NamespaceError {
    reason: NamespaceErrorType,
    invalid_index: usize,
}

impl NamespaceError {
    pub fn new(reason: NamespaceErrorType, invalid_index: usize) -> Self {
        Self {
            reason,
            invalid_index,
        }
    }

    pub fn reason(&self) -> NamespaceErrorType {
        self.reason
    }

    pub fn invalid_index(&self) -> usize {
        self.invalid_index
    }
}

impl From<TopicNameError> for NamespaceError {
    fn from(err: TopicNameError) -> Self {
        match err.reason() {
            TopicNameErrorType::EmptyString => {
                NamespaceError::new(NamespaceErrorType::EmptyString, err.invalid_index())
            }
            TopicNameErrorType::NotAbsolute => {
                NamespaceError::new(NamespaceErrorType::NotAbsolute, err.invalid_index())
            }
            TopicNameErrorType::EndsWithForwardSlash => NamespaceError::new(
                NamespaceErrorType::EndsWithForwardSlash,
                err.invalid_index(),
            ),
            TopicNameErrorType::ContainsUnallowedChars => NamespaceError::new(
                NamespaceErrorType::ContainsUnallowedChars,
                err.invalid_index(),
            ),
            TopicNameErrorType::ContainsRepeatedForwardSlash => NamespaceError::new(
                NamespaceErrorType::ContainsRepeatedForwardSlash,
                err.invalid_index(),
            ),
            TopicNameErrorType::StartsWithNumber => {
                NamespaceError::new(NamespaceErrorType::StartsWithNumber, err.invalid_index())
            }
            TopicNameErrorType::TooLong => {
                NamespaceError::new(NamespaceErrorType::TooLong, err.invalid_index())
            }
        }
    }
}

/// Determine if a given namespace is valid.
///
/// The `NamespaceErrorType::TooLong` is guaranteed to be checked last,
/// such that if you get that result, then you can assume all other checks
/// succeeded.
/// This is done so that the length limit can be treated as a warning rather
/// than an error if desired.
pub fn validate_namespace(namespace: &str) -> Result<(), NamespaceError> {
    // Special case for root namespace
    if namespace == "/" {
        return Ok(());
    }

    // All other cases should pass the validate topic name test.
    if let Err(err) = validate_topic_name(namespace) {
        let err: NamespaceError = err.into();
        // Ignores `TooLong` error as it will be rechecked below.
        if err.reason() != NamespaceErrorType::TooLong {
            return Err(err);
        }
    }

    // check if the namespace is too long last, since it might be a soft invalidation.
    if namespace.len() > NAMESPACE_MAX_LENGTH {
        return Err(NamespaceError::new(
            NamespaceErrorType::TooLong,
            NAMESPACE_MAX_LENGTH - 1,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_namespace() {
        assert!(validate_namespace("/").is_ok());
        assert!(validate_namespace("/basename_only").is_ok());
        assert!(validate_namespace("/with_one/hierarchy").is_ok());
    }

    #[test]
    fn test_empty_namespace() {
        assert_eq!(
            validate_namespace(""),
            Err(NamespaceError::new(NamespaceErrorType::EmptyString, 0))
        );
    }

    #[test]
    fn test_not_absolute() {
        assert_eq!(
            validate_namespace("not_absolute"),
            Err(NamespaceError::new(NamespaceErrorType::NotAbsolute, 0))
        );

        assert_eq!(
            validate_namespace("not/absolute"),
            Err(NamespaceError::new(NamespaceErrorType::NotAbsolute, 0))
        );
    }

    #[test]
    fn test_ends_with_forward_slash() {
        assert_eq!(
            validate_namespace("/ends/with/"),
            Err(NamespaceError::new(
                NamespaceErrorType::EndsWithForwardSlash,
                10
            ))
        );
    }

    #[test]
    fn test_unallowed_characters() {
        assert_eq!(
            validate_namespace("/~/unexpanded_tilde"),
            Err(NamespaceError::new(
                NamespaceErrorType::ContainsUnallowedChars,
                1
            ))
        );

        assert_eq!(
            validate_namespace("/unexpanded_sub/{node}"),
            Err(NamespaceError::new(
                NamespaceErrorType::ContainsUnallowedChars,
                16
            ))
        );

        assert_eq!(
            validate_namespace("/question?"),
            Err(NamespaceError::new(
                NamespaceErrorType::ContainsUnallowedChars,
                9
            ))
        );

        assert_eq!(
            validate_namespace("/with spaces"),
            Err(NamespaceError::new(
                NamespaceErrorType::ContainsUnallowedChars,
                5
            ))
        );
    }

    #[test]
    fn test_repeated_forward_slashes() {
        assert_eq!(
            validate_namespace("/repeated//slashes"),
            Err(NamespaceError::new(
                NamespaceErrorType::ContainsRepeatedForwardSlash,
                10
            ))
        );
    }

    #[test]
    fn test_starts_with_number() {
        assert_eq!(
            validate_namespace("/9starts_with_number"),
            Err(NamespaceError::new(NamespaceErrorType::StartsWithNumber, 1))
        );

        assert_eq!(
            validate_namespace("/starts/42with/number"),
            Err(NamespaceError::new(NamespaceErrorType::StartsWithNumber, 8))
        );
    }

    #[test]
    fn test_topic_too_long() {
        let invalid_long_topic: String = "a".repeat(NAMESPACE_MAX_LENGTH + 1);
        assert_eq!(
            validate_namespace(&invalid_long_topic),
            Err(NamespaceError::new(NamespaceErrorType::NotAbsolute, 0))
        );

        let valid_long_topic = "/".to_owned() + &invalid_long_topic;
        assert_eq!(
            validate_namespace(&valid_long_topic),
            Err(NamespaceError::new(
                NamespaceErrorType::TooLong,
                NAMESPACE_MAX_LENGTH - 1
            ))
        );
    }
}
