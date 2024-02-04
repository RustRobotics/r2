// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub trait MessageTypeSupportTrait {
    /// String identifier for the type_support.
    fn typesupport_identifier(&self) -> &str;

    // Pointer to the message type support library
    //const void * data;

    // Pointer to the message type support handler function
    //rosidl_message_typesupport_handle_function func;
}
