// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// An additional 2 characters are reserved for the shortest possible topic, e.g. '/X'.
pub const NAMESPACE_MAX_LENGTH: usize = 255 - 8;

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
    pub reason: NamespaceErrorType,
    pub invalid_index: usize,
}

pub fn validate_namespace(namespace: &str) -> Result<(), NamespaceError> {
    Ok(())
}
