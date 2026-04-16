// Copyright (C) 2026 wgrav
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub enum TokenKind {
    // Basic syntax
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,

    // Operators, precedence level 1
    New, // Creation
    Bang, // "!", logical NOT
    Tilde, // "~", bitwise NOT

    // Operators, precedence level 2
    Star, // "*", multiplication
    Slash, // "/", division
    Percent, // "%", modulo
    Ampersand, // "&", bitwise and
    DoubleLess, // "<<", left shift
    DoubleGreater, // "<<", right shift    
    
    // Operators, precedence level 3
    Plus, // "+", addittion
    Minus, // "-", subtraction
    VerticalBar, // "|", bitwise OR
    Caret, // "^", bitwise XOR

    // Operators, precedence level 4
    Less, // "<", less than
    LessEqual, // "<=", less than or equals
    Greater, // ">", greater than
    GreaterEqual, // ">=", greater than or equals
    EqualEqual, // "==", equal equals
    BangEqual, // "!=", not equal

    // Operators, precedence level 5
    DoubleVerticalBar, // "||", logical or

    // Operators, precedence level 6
    QuestionColon, // "?:", conditional

    // Literals
    Identifier, // Variable
    Int, // 32-bit signed integers
    Float, // 32-bit floating point numbers
    Long, // 64-bit signed integers
    Double, // 64-bit floating point numbers
    Boolean, // true and false
    Char, // Unicode characters
    Str, // Strings of characters
    Object, // Instantiated objects (defined with the class keyword)
    Array, // Allocated with the syntax new [X] where 'X' is an expression computing the size of the array
    Dictionary, // Associative arrays, allocated with the syntax {}

    // Keywords
    As,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Else,
    Enum,
    Extends,
    Finally,
    For,
    Function,
    Has,
    Hidden,
    If,
    InstanceOf,
    Me,
    Module,
    Private,
    Protected,
    Public,
    Return,
    SelfKeyword, // TODO: find a better (not reserved) name
    Static,
    Switch,
    Throw,
    Try,
    Using,
    Var,
    While
}