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

use phf::phf_map;

#[derive(Debug, Clone, PartialEq)]
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

pub static KEYWORDS: phf::Map<&str, TokenKind> = phf_map! {
    "as" => TokenKind::As,
    "break" => TokenKind::Break,
    "case" => TokenKind::Case,
    "catch" => TokenKind::Catch,
    "const" => TokenKind::Const,
    "continue" => TokenKind::Continue,
    "default" => TokenKind::Default,
    "do" => TokenKind::Do,
    "else" => TokenKind::Else,
    "enum" => TokenKind::Enum,
    "extends" => TokenKind::Extends,
    "finally" => TokenKind::Finally,
    "for" => TokenKind::For,
    "function" => TokenKind::Function,
    "has" => TokenKind::Has,
    "hidden" => TokenKind::Hidden,
    "if" => TokenKind::If,
    "instanceof" => TokenKind::InstanceOf,
    "me" => TokenKind::Me,
    "module" => TokenKind::Module,
    "private" => TokenKind::Private,
    "protected" => TokenKind::Protected,
    "public" => TokenKind::Public,
    "return" => TokenKind::Return,
    "self" => TokenKind::SelfKeyword,
    "static" => TokenKind::Static,
    "switch" => TokenKind::Switch,
    "throw" => TokenKind::Throw,
    "try" => TokenKind::Try,
    "using" => TokenKind::Using,
    "var" => TokenKind::Var,
    "while" => TokenKind::While
};

#[cfg(test)]
mod keyword_matching_tests {
    use super::*;

    #[test]
    fn as_keyword() {
        assert_eq!(KEYWORDS.get("as").cloned(), Some(TokenKind::As))
    }

    #[test]
    fn break_keyword() {
        assert_eq!(KEYWORDS.get("break").cloned(), Some(TokenKind::Break))
    }
    
    #[test]
    fn case_keyword() {
        assert_eq!(KEYWORDS.get("case").cloned(), Some(TokenKind::Case))
    }

    #[test]
    fn catch_keyword() {
        assert_eq!(KEYWORDS.get("catch").cloned(), Some(TokenKind::Catch))
    }

    #[test]
    fn const_keyword() {
        assert_eq!(KEYWORDS.get("const").cloned(), Some(TokenKind::Const))
    }

    #[test]
    fn continue_keyword() {
        assert_eq!(KEYWORDS.get("continue").cloned(), Some(TokenKind::Continue))
    }

    #[test]
    fn default_keyword() {
        assert_eq!(KEYWORDS.get("default").cloned(), Some(TokenKind::Default))
    }

    #[test]
    fn do_keyword() {
        assert_eq!(KEYWORDS.get("do").cloned(), Some(TokenKind::Do))
    }

    #[test]
    fn else_keyword() {
        assert_eq!(KEYWORDS.get("else").cloned(), Some(TokenKind::Else))
    }

    #[test]
    fn enum_keyword() {
        assert_eq!(KEYWORDS.get("enum").cloned(), Some(TokenKind::Enum))
    }

    #[test]
    fn extends_keyword() {
        assert_eq!(KEYWORDS.get("extends").cloned(), Some(TokenKind::Extends))
    }

    #[test]
    fn finally_keyword() {
        assert_eq!(KEYWORDS.get("finally").cloned(), Some(TokenKind::Finally))
    }

    #[test]
    fn for_keyword() {
        assert_eq!(KEYWORDS.get("for").cloned(), Some(TokenKind::For))
    }

    #[test]
    fn function_keyword() {
        assert_eq!(KEYWORDS.get("function").cloned(), Some(TokenKind::Function))
    }
    
    #[test]
    fn has_keyword() {
        assert_eq!(KEYWORDS.get("has").cloned(), Some(TokenKind::Has))
    }

    #[test]
    fn hidden_keyword() {
        assert_eq!(KEYWORDS.get("hidden").cloned(), Some(TokenKind::Hidden))
    }

    #[test]
    fn if_keyword() {
        assert_eq!(KEYWORDS.get("if").cloned(), Some(TokenKind::If))
    }

    #[test]
    fn instanceof_keyword() {
        assert_eq!(KEYWORDS.get("instanceof").cloned(), Some(TokenKind::InstanceOf))
    }

    #[test]
    fn me_keyword() {
        assert_eq!(KEYWORDS.get("me").cloned(), Some(TokenKind::Me))
    }
    
    #[test]
    fn module_keyword() {
        assert_eq!(KEYWORDS.get("module").cloned(), Some(TokenKind::Module))
    }
    
    #[test]
    fn private_keyword() {
        assert_eq!(KEYWORDS.get("private").cloned(), Some(TokenKind::Private))
    }
    
    #[test]
    fn protected_keyword() {
        assert_eq!(KEYWORDS.get("protected").cloned(), Some(TokenKind::Protected))
    }

    #[test]
    fn public_keyword() {
        assert_eq!(KEYWORDS.get("public").cloned(), Some(TokenKind::Public))
    }

    #[test]
    fn return_keyword() {
        assert_eq!(KEYWORDS.get("return").cloned(), Some(TokenKind::Return))
    }
    
    #[test]
    fn self_keyword() {
        assert_eq!(KEYWORDS.get("self").cloned(), Some(TokenKind::SelfKeyword))
    }
    
    #[test]
    fn static_keyword() {
        assert_eq!(KEYWORDS.get("static").cloned(), Some(TokenKind::Static))
    }
    
    #[test]
    fn switch_keyword() {
        assert_eq!(KEYWORDS.get("switch").cloned(), Some(TokenKind::Switch))
    }
    
    #[test]
    fn throw_keyword() {
        assert_eq!(KEYWORDS.get("throw").cloned(), Some(TokenKind::Throw))
    }

    #[test]
    fn try_keyword() {
        assert_eq!(KEYWORDS.get("try").cloned(), Some(TokenKind::Try))
    }

    #[test]
    fn using_keyword() {
        assert_eq!(KEYWORDS.get("using").cloned(), Some(TokenKind::Using))
    }
    
    #[test]
    fn var_keyword() {
        assert_eq!(KEYWORDS.get("var").cloned(), Some(TokenKind::Var))
    }

    #[test]
    fn while_keyword() {
        assert_eq!(KEYWORDS.get("while").cloned(), Some(TokenKind::While))
    }

    #[test]
    fn no_match() {
        assert_eq!(KEYWORDS.get("This isn't a keyword").cloned(), None)
    }
}
