// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Serialized message as a string of bytes.
#[derive(Debug, Default, Clone)]
pub struct SerializedMessage(Vec<u8>);

impl SerializedMessage {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Resizes the `SerializedMessage` in-place so that `len` is equal to `new_len`.
    #[inline]
    pub fn resize(&mut self, new_size: usize) {
        self.0.resize(new_size, 0);
    }

    /// Returns the number of elements in the message, also referred to as its ‘length’.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
