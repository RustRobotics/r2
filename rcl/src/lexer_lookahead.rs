// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Debug;

/// Forward declaration
pub trait LexerLookahead2Impl: Debug {}

/// Track lexical analysis and allow looking ahead 2 lexemes.
#[derive(Debug)]
pub struct LexerLookahead2 {
    /// Pointer to the lexer look ahead2 implementation
    pub imp: Box<dyn LexerLookahead2Impl>,
}
