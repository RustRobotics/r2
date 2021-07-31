// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use pest::Parser;

#[derive(Parser)]
#[grammar = "msg.pest"]
pub struct MsgParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primitive_type_name() {
        assert!(MsgParser::parse(Rule::primitive_type_name, "int32").is_ok());
        assert!(MsgParser::parse(Rule::primitive_type_name, "int").is_err());
    }
}
