// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Associative array of topic or service names and types.
#[derive(Debug)]
pub struct NamesAndTypes {
    /// Array of names
    pub names: Vec<String>,

    /// Dynamic array of arrays of type names, with the same length as `names`
    // string_array_t * types;
    pub types: Vec<String>,
}
