// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum NamespaceErrorType {
    /// Node name is empty.
    EmptyString,

    NotAbsolute,

    EndsWithForwardSlash,

    /// Only alphanumeric characters and underscores are allowed.
    ContainsUnallowedChars,

    ContainsRepeatedForwardSlash,

    /// Must not start with a number.
    StartsWithNumber,

    /// Length of node name is larger than 255.
    TooLong,
}

#[derive(Debug, Clone, Copy)]
pub struct NamespaceError {
    pub reason: NamespaceErrorType,
    pub invalid_index: usize,
}

pub fn validate_namespace(namespace: &str) -> Result<(), NamespaceError> {
    Ok(())
}
