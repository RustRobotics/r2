// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Messages are described and defined in `.msg` files in the `msg/` directory of a R2 package.
/// `.msg` files are composed of two parts: fields and constants.
#[derive(Parser)]
#[grammar = "msg.pest"]
pub struct MsgParser;

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_primitive_type_name() {
        assert!(MsgParser::parse(Rule::primitive_type_name, "int32").is_ok());
        assert!(MsgParser::parse(Rule::primitive_type_name, "int").is_err());
    }

    #[test]
    fn test_parse_complex_type_name() {
        assert!(MsgParser::parse(Rule::complex_type_name, "bounded string").is_ok());
        assert!(MsgParser::parse(Rule::complex_type_name, "vector").is_err());
    }

    #[test]
    fn test_parse_variable_name() {
        assert!(MsgParser::parse(Rule::variable_name, "client_id").is_ok());
        assert!(MsgParser::parse(Rule::variable_name, "Var1").is_err());
    }

    #[test]
    fn test_parse_default_value() {
        assert!(MsgParser::parse(Rule::default_value, "42").is_ok());
        assert!(MsgParser::parse(Rule::default_value, "word").is_err());
    }

    #[test]
    fn test_parse_constant_name() {
        assert!(MsgParser::parse(Rule::constant_name, "CLIENT_ID").is_ok());
        assert!(MsgParser::parse(Rule::constant_name, "Var1").is_err());
    }
}
