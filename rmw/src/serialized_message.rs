// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::ret_types::RetType;

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

pub trait SerializedMessageTrait {
    /// Compute the size of a serialized message.
    fn get_serialized_message_size(
        //const rosidl_message_type_support_t * type_support,
        //const rosidl_runtime_c__Sequence__bound * message_bounds,
        size: &mut usize,
    ) -> RetType;

    /// Serialize an R2 message into a SerializedMessage.
    fn serialize(
        r2_message: *const u8,
        //const rosidl_message_type_support_t * type_support,
        serialized_message: &mut SerializedMessage,
    ) -> RetType;

    /// Deserialize a ROS message.
    //TODO(Shaohua): Replace usize with &mut [u8]
    fn deserialize(
        serialized_message: &SerializedMessage,
        //const rosidl_message_type_support_t * type_support,
        r2_message: usize,
    ) -> RetType;
}
