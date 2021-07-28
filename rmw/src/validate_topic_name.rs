// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Length of topic name must not exceed `255 - 8` characters.
/// `8` characters are reversed for prefixes.
pub const TOPIC_NAME_MAX_NAME_LENGTH: usize = 255 - 8;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum TopicNameErrorType {
    /// Topic name is empty.
    EmptyString,

    /// Not starts with `/`.
    NotAbsolute,

    /// Must not ends with `/`.
    EndsWithForwardSlash,

    /// Only alphanumeric characters, underscores and slashes are allowed.
    ContainsUnallowedChars,

    ContainsRepeatedForwardSlash,

    /// Must not start with a number.
    /// Slashes must not be followed with numbers.
    StartsWithNumber,

    /// Length of topic name is larger than `255 - 8`.
    TooLong,
}

#[derive(Debug, Clone, Copy)]
pub struct TopicNameError {
    pub reason: TopicNameErrorType,
    pub invalid_index: usize,
}

impl TopicNameError {
    pub fn new(reason: TopicNameErrorType, invalid_index: usize) -> Self {
        Self {
            reason,
            invalid_index,
        }
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
    if topic_name.len() > TOPIC_NAME_MAX_NAME_LENGTH {
        return Err(TopicNameError::new(
            TopicNameErrorType::TooLong,
            TOPIC_NAME_MAX_NAME_LENGTH - 1,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_topic() {
        assert!(validate_topic_name("/basename_only").is_ok());
        assert!(validate_topic_name("/with_one/namespace").is_ok());
        assert!(validate_topic_name("/with_double/namespaces/sep").is_ok());
    }
}
