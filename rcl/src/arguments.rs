// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

/// Hold output of parsing command line arguments.
#[derive(Debug)]
pub struct Arguments {
    /// Private implementation pointer.
    pub imp: Box<dyn ArgumentsImpl>,
}

pub trait ArgumentsImpl: Debug {}

/// The command-line flag that delineates the start of R2 arguments.
pub const ARGS_FLAG: &str = "--r2-args";

/// The token that delineates the explicit end of R2 arguments.
pub const ARGS_EXPLICIT_END_TOKEN: &str = "--";

/// The R2 flag that precedes the setting of a R2 parameter.
pub const PARAM_FLAG: &str = "--param";

/// The short version of the R2 flag that precedes the setting of a R2 parameter.
pub const SHORT_PARAM_FLAG: &str = "-p";

/// The R2 flag that precedes a path to a file containing R2 parameters.
pub const PARAM_FILE_FLAG: &str = "--params-file";

/// The R2 flag that precedes a R2 remapping rule.
pub const REMAP_FLAG: &str = "--remap";

/// The short version of the R2 flag that precedes a R2 remapping rule.
pub const SHORT_REMAP_FLAG: &str = "-r";

/// The R2 flag that precedes the name of a R2 security enclave.
pub const ENCLAVE_FLAG: &str = "--enclave";

/// The short version of the R2 flag that precedes the name of a R2 security enclave.
pub const SHORT_ENCLAVE_FLAG: &str = "-e";

/// The R2 flag that precedes the R2 logging level to set.
pub const LOG_LEVEL_FLAG: &str = "--log-level";

/// The R2 flag that precedes the name of a configuration file to configure logging.
pub const EXTERNAL_LOG_CONFIG_FLAG: &str = "--log-config-file";

/// The suffix of the R2 flag to enable or disable stdout
/// logging (must be preceded with --enable- or --disable-).
pub const LOG_STDOUT_FLAG_SUFFIX: &str = "stdout-logs";

/// The suffix of the R2 flag to enable or disable r2out
/// logging (must be preceded with --enable- or --disable-).
pub const LOG_R2OUT_FLAG_SUFFIX: &str = "r2out-logs";

/// The suffix of the R2 flag to enable or disable external library
/// logging (must be preceded with --enable- or --disable-).
pub const LOG_EXT_LIB_FLAG_SUFFIX: &str = "external-lib-logs";
