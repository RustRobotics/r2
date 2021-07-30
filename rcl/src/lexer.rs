// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Type of lexeme found by lexical analysis.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lexeme {
    /// Indicates no valid lexeme was found (end of input not reached)
    None = 0,

    /// Indicates end of input has been reached
    EOF = 1,

    /// ~/
    TildeSlash = 2,

    /// rosservice://
    UrlService = 3,

    /// rostopic://
    UrlTopic = 4,

    /// :
    Colon = 5,

    /// __node or __name
    Node = 6,

    /// __ns
    Ns = 7,

    /// :=
    Separator = 8,

    /// \1
    Br1 = 9,

    /// \2
    Br2 = 10,

    /// \3
    Br3 = 11,

    /// \4
    Br4 = 12,

    /// \5
    Br5 = 13,

    /// \6
    Br6 = 14,

    /// \7
    Br7 = 15,

    /// \8
    Br8 = 16,

    /// \9
    Br9 = 17,

    /// a name between slashes, must match (([a-zA-Z](_)?)|_)([0-9a-zA-Z](_)?)*
    Token = 18,

    /// /
    ForwardSlash = 19,

    /// *
    WildOne = 20,

    /// **
    WildMulti = 21,

    /// \.
    Dot = 22,
}
