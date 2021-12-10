// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

use crate::domain_id::{DomainId, DEFAULT_DOMAIN_ID};
use crate::init::ContextTrait;
use crate::localhost::LocalhostOnly;
use crate::ret_types::{self, RetType};
use crate::security_options::SecurityOptions;

/// Options structure used during [`ContextTrait::init()`].
pub trait InitOptionsBaseTrait<'a> {
    /// Locally (process local) unique ID that represents this init/shutdown cycle.
    ///
    /// This should be set by the caller of [`ContextTrait::init()`] to a number that is
    /// unique within this process.
    ///
    /// It is designed to be used with [`ContextTrait::init()`] and `rcl_get_instance_id()`.
    fn instance_id(&self) -> u64;

    /// Implementation identifier, used to ensure two different implementations are not being mixed.
    fn implementation_identifier(&self) -> &'a str;

    /// R2 domain id
    fn domain_id(&self) -> DomainId;

    /// Security options
    fn security_options(&self) -> &SecurityOptions;

    /// Enable localhost only
    fn localhost_only(&self) -> LocalhostOnly;

    /// Enclave, name used to find security artifacts in a sr2 keystore.
    fn enclave(&self) -> &'a str;

    /// Return a zero initialized init options structure.
    fn zero_initialized() -> Self;
}

pub trait InitOptionsTrait<'a>: InitOptionsBaseTrait<'a> {
    /// Initialize given init options with the default values and implementation specific values.
    ///
    /// The given init options must be zero initialized.
    /// If initialization fails, init options will remain zero initialized.
    /// Giving an already initialized init options will result in a failure
    /// with return code [`ret_types::RET_INVALID_ARGUMENT`].
    fn init(&mut self) -> RetType;

    /// Copy the given source init options to the destination init options.
    fn copy(self: &Self, dest: &mut Self) -> RetType;
}
