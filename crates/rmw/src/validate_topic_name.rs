// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Length of topic name must not exceed `255 - 8` characters.
/// `8` characters are reversed for prefixes.
pub const TOPIC_NAME_MAX_LENGTH: usize = 255 - 8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TopicNameErrorType {
    /// Topic name is empty.
    EmptyString,

    /// Not starts with `/`.
    NotAbsolute,

    /// Must not ends with `/`.
    EndsWithForwardSlash,

    /// Only alphanumeric characters, underscores and slashes are allowed.
    ContainsUnallowedChars,

    /// A slash must not be followed by another slash.
    ContainsRepeatedForwardSlash,

    /// Must not start with a number.
    /// A slash must not be followed by a number.
    StartsWithNumber,

    /// Length of topic name is larger than `255 - 8`.
    TooLong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopicNameError {
    reason: TopicNameErrorType,
    invalid_index: usize,
}

impl TopicNameError {
    pub fn new(reason: TopicNameErrorType, invalid_index: usize) -> Self {
        Self {
            reason,
            invalid_index,
        }
    }

    pub fn reason(&self) -> TopicNameErrorType {
        self.reason
    }

    pub fn invalid_index(&self) -> usize {
        self.invalid_index
    }
}

/// Determine if a given fully qualified topic name is valid.
///
/// The `TopicNameErrorType::TooLong` is guaranteed to be checked last, such
/// that if you get that result, then you can assume all other checks succeeded.
/// This is done so that the length limit can be treated as a warning rather
/// than an error if desired.
///
/// Topic names must follow these rules:
///   - must not be an empty string
///   - must only contain alphanumeric characters, underscores and slashes (a-z|A-Z|0-9|_/)
///   - must not start with a number
///   - must start with `/`
///   - must not end with `/`
pub fn validate_topic_name(topic_name: &str) -> Result<(), TopicNameError> {
    if topic_name.is_empty() {
        return Err(TopicNameError::new(TopicNameErrorType::EmptyString, 0));
    }

    if !topic_name.starts_with('/') {
        return Err(TopicNameError::new(TopicNameErrorType::NotAbsolute, 0));
    }

    // Catches both '/foo/' and '/'.
    if topic_name.ends_with('/') {
        return Err(TopicNameError::new(
            TopicNameErrorType::EndsWithForwardSlash,
            topic_name.len() - 1,
        ));
    }

    // check for unallowed characters.
    for (idx, c) in topic_name.chars().enumerate() {
        if !(c.is_ascii_alphanumeric() || c == '_' || c == '/') {
            return Err(TopicNameError::new(
                TopicNameErrorType::ContainsUnallowedChars,
                idx,
            ));
        }
    }

    // Check for double '/' and tokens that start with a number.
    let mut last_char_is_slash = false;
    for (idx, c) in topic_name.chars().enumerate() {
        if last_char_is_slash {
            if c == '/' {
                return Err(TopicNameError::new(
                    TopicNameErrorType::ContainsRepeatedForwardSlash,
                    idx,
                ));
            } else if c.is_digit(10) {
                // This is the case where a '/' if followed by a number, i.e. [0-9].
                return Err(TopicNameError::new(
                    TopicNameErrorType::StartsWithNumber,
                    idx,
                ));
            }
        }

        last_char_is_slash = c == '/';
    }

    // Check if the topic name is too long last, since it might be a soft invalidation.
    if topic_name.len() > TOPIC_NAME_MAX_LENGTH {
        return Err(TopicNameError::new(
            TopicNameErrorType::TooLong,
            TOPIC_NAME_MAX_LENGTH - 1,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_topic() {
        assert!(validate_topic_name("/basename_only").is_ok());
        assert!(validate_topic_name("/with_one/namespace").is_ok());
        assert!(validate_topic_name("/with_double/namespaces/sep").is_ok());
    }

    #[test]
    fn test_empty_topic_name() {
        assert_eq!(
            validate_topic_name(""),
            Err(TopicNameError::new(TopicNameErrorType::EmptyString, 0))
        );
    }

    #[test]
    fn test_not_absolute() {
        assert_eq!(
            validate_topic_name("not_absolute"),
            Err(TopicNameError::new(TopicNameErrorType::NotAbsolute, 0))
        );

        assert_eq!(
            validate_topic_name("not/absolute"),
            Err(TopicNameError::new(TopicNameErrorType::NotAbsolute, 0))
        );
    }

    #[test]
    fn test_ends_with_forward_slash() {
        assert_eq!(
            validate_topic_name("/ends/with/"),
            Err(TopicNameError::new(
                TopicNameErrorType::EndsWithForwardSlash,
                10
            ))
        );

        assert_eq!(
            validate_topic_name("/"),
            Err(TopicNameError::new(
                TopicNameErrorType::EndsWithForwardSlash,
                0
            ))
        );
    }

    #[test]
    fn test_unallowed_characters() {
        assert_eq!(
            validate_topic_name("/~/unexpected_tilde"),
            Err(TopicNameError::new(
                TopicNameErrorType::ContainsUnallowedChars,
                1
            ))
        );

        assert_eq!(
            validate_topic_name("/unexpected_sub/{node}"),
            Err(TopicNameError::new(
                TopicNameErrorType::ContainsUnallowedChars,
                16
            ))
        );

        assert_eq!(
            validate_topic_name("/question?"),
            Err(TopicNameError::new(
                TopicNameErrorType::ContainsUnallowedChars,
                9
            ))
        );

        assert_eq!(
            validate_topic_name("/with spaces"),
            Err(TopicNameError::new(
                TopicNameErrorType::ContainsUnallowedChars,
                5
            ))
        );
    }

    #[test]
    fn test_repeated_forward_slashes() {
        assert_eq!(
            validate_topic_name("/repeated//slashes"),
            Err(TopicNameError::new(
                TopicNameErrorType::ContainsRepeatedForwardSlash,
                10
            ))
        );
    }

    #[test]
    fn test_starts_with_number() {
        assert_eq!(
            validate_topic_name("/9starts_with_number"),
            Err(TopicNameError::new(TopicNameErrorType::StartsWithNumber, 1))
        );

        assert_eq!(
            validate_topic_name("/starts/42with/number"),
            Err(TopicNameError::new(TopicNameErrorType::StartsWithNumber, 8))
        );
    }

    #[test]
    fn test_topic_too_long() {
        let invalid_long_topic: String = "a".repeat(TOPIC_NAME_MAX_LENGTH + 1);
        assert_eq!(
            validate_topic_name(&invalid_long_topic),
            Err(TopicNameError::new(TopicNameErrorType::NotAbsolute, 0))
        );

        let valid_but_long_topic = "/".to_owned() + &invalid_long_topic;
        assert_eq!(
            validate_topic_name(&valid_but_long_topic),
            Err(TopicNameError::new(
                TopicNameErrorType::TooLong,
                TOPIC_NAME_MAX_LENGTH - 1
            ))
        );
    }
}
