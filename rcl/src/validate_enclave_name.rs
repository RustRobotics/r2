// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use rmw::validate_namespace::NamespaceErrorType;
use rmw::validate_node_name::NODE_NAME_MAX_LENGTH;

pub type EnclaveNameErrorType = NamespaceErrorType;

/// The maximum length of an enclave name.
pub const ENCLAVE_NAME_MAX_LENGTH: usize = NODE_NAME_MAX_LENGTH;

#[derive(Debug)]
pub struct EnclaveNameError {
    pub kind: EnclaveNameErrorType,
    pub invalid_index: usize,
}
