// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::arguments::Arguments;
use super::domain_id::DEFAULT_DOMAIN_ID;
use rmw::qos_profiles::QoSProfile;

/// Constant which indicates that the default domain id should be used.
pub const NODE_OPTIONS_DEFAULT_DOMAIN_ID: u64 = DEFAULT_DOMAIN_ID;

/// Structure which encapsulates the options for creating a `Node`.
#[derive(Debug)]
pub struct NodeOptions {
    //pub anonymous_name: bool,

    //pub parameter_qos: QoSProfile,
    /// If true, no parameter infrastructure will be setup.
    //pub no_parameters: bool,

    /// If false then only use arguments in this struct, otherwise use global arguments also.
    pub use_global_arguments: bool,

    /// Command line arguments that apply only to this node.
    pub arguments: Arguments,

    /// Flag to enable rosout for this node
    pub enable_rosout: bool,

    /// Middleware quality of service settings for /rosout.
    pub rosout_qos: QoSProfile,
}
