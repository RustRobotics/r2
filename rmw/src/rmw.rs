// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Root trait to wrapper global or general methods.
pub trait RmwTrait {
    /// Get the name of the rmw implementation being used.
    ///
    /// Return Name of rmw implementation.
    fn get_implementation_identifier() -> &str;

    /// Get the unique serialization format for this middleware.
    ///
    /// Return the format in which binary data is serialized.
    /// One middleware can only have one encoding.
    /// In contrast to the implementation identifier, the serialization format can be equal between
    /// multiple RMW implementations.
    /// This means, that the same binary messages can be deserialized by RMW implementations with the
    /// same format.
    ///
    /// Return serialization format.
    fn get_serialization_format() -> &str;
}
