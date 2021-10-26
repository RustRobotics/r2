// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::ret_types;

/// Associative array of topic or service names and types.
#[derive(Debug, Default)]
pub struct NamesAndTypes {
    /// Array of names
    pub names: Vec<String>,

    /// Dynamic array of arrays of type names, with the same length as `names`
    // string_array_t * types;
    pub types: Vec<String>,
}

impl NamesAndTypes {
    /// Return a zero initialized array of names and types.
    pub fn zero_initialized() -> Self {
        Self::default()
    }

    /// Check that the given `names_and_types` array is zero initialized.
    pub fn check_zero(&self) -> ret_types::RetType {
        if !self.names.is_empty() {
            log::error!("names array is not zeroed");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        if !self.types.is_empty() {
            log::error!("types array is not NULL");
            return ret_types::RET_INVALID_ARGUMENT;
        }
        ret_types::RET_OK
    }

    /// Initialize an array of names and types.
    ///
    /// This function initializes the string array for the names and allocates space
    /// for all the string arrays for the types according to the given size, but
    /// it does not initialize the string array for each setup of types.
    /// However, the string arrays for each set of types is zero initialized.
    pub fn init(&mut self, size: usize) -> ret_types::RetType {
        self.names.resize_with(size, Default::default);
        self.types.resize_with(size, Default::default);
        ret_types::RET_OK
    }
}
