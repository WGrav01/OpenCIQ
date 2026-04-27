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

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum TokenKind {
    // Basic syntax
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,

    /// "!", logical NOT
    Bang,

    /// "~", bitwise NOT   y)
    Tilde,

    /// "*", multiplication
    Star,

    /// "/", division   
    Slash,
    /// "%", modulo
    Percent,

    /// "&", bitwise and   
    Ampersand,
    /// "<<", left shift   
    DoubleLess,

    DoubleGreater, // "<<", right shift

    /// "+", addittion
    Plus,

    /// "-", subtraction           
    Minus,

    /// "|", bitwise OR
    VerticalBar,

    /// "^", bitwise XOR
    Caret,

    /// "<", less than
    Less,

    /// "<=", less than or equals    
    LessEqual,

    /// ">", greater than   
    Greater,

    /// ">=", greater than or equals    
    GreaterEqual,

    /// '=', for setting vars
    Equal,

    /// "==", equal equals
    EqualEqual,

    BangEqual,

    /// "||", logical or
    DoubleVerticalBar,

    /// "?:", conditional
    QuestionColon,

    /// Variable, function name, etc
    Identifier,

    /// Literal float or integer
    Number,

    /// Literal boolean (true, or false)
    Boolean,

    /// String literal
    String,

    /// Logical AND keyword, considered operator of precedence level 5
    And,
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
    /// Creation, considered operator of precedence level 1
    New,
    /// Logical OR keyword, considered operator of precedence level 6
    Or,
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
    While,
}

pub static KEYWORDS: phf::Map<&str, TokenKind> = phf_map! {
    "and" => TokenKind::And,
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
    "false" => TokenKind::Boolean,
    "finally" => TokenKind::Finally,
    "for" => TokenKind::For,
    "function" => TokenKind::Function,
    "has" => TokenKind::Has,
    "hidden" => TokenKind::Hidden,
    "if" => TokenKind::If,
    "instanceof" => TokenKind::InstanceOf,
    "me" => TokenKind::Me,
    "module" => TokenKind::Module,
    "or" => TokenKind::Or,
    "private" => TokenKind::Private,
    "protected" => TokenKind::Protected,
    "public" => TokenKind::Public,
    "return" => TokenKind::Return,
    "self" => TokenKind::SelfKeyword,
    "static" => TokenKind::Static,
    "switch" => TokenKind::Switch,
    "throw" => TokenKind::Throw,
    "true" => TokenKind::Boolean,
    "try" => TokenKind::Try,
    "using" => TokenKind::Using,
    "var" => TokenKind::Var,
    "while" => TokenKind::While,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Variable name. Might get removed from this enum
    Identifier(String),

    /// Can be any type, float, int, etc.
    Number(String),

    /// true and false
    Boolean(bool),

    /// Unicode characters
    Char(char),

    /// Strings of characters. Note: the String type might change
    Str(String),

    /// Instantiated objects (defined with the class keyword)
    Object, // TODO: get type for this

    /// Allocated with the syntax new [X] where 'X' is an expression computing the size of the array.
    Array, // TODO: Get type for this

    /// Associative arrays, allocated with the syntax {}.
    Dictionary, // TODO: Get type for this

    /// No literal
    Nil,
}

/// A structure of arrays containing tokens
#[derive(Debug)]
pub struct TokenPool {
    /// The type of the token
    kind: Vec<TokenKind>,

    /// The literal value of the token. Nil if not applicable
    literal: Vec<Literal>,

    /// The line number of the token
    line: Vec<usize>,

    /// The starting position of the token on the line
    pos: Vec<usize>,
}

impl TokenPool {
    /// Returns a brand spankin' new TokenPool, pre allocated at the capacity var.
    /// Reccomended to set capacity to an guesstimate of the tokens (i.e source char length / 4).
    pub fn new(capacity: usize) -> TokenPool {
        TokenPool {
            kind: Vec::with_capacity(capacity),
            literal: Vec::with_capacity(capacity),
            line: Vec::with_capacity(capacity),
            pos: Vec::with_capacity(capacity),
        }
    }

    /// Returns the length of the tokens in the TokenPool.
    pub fn len(&self) -> usize {
        self.kind.len() // safe, as the fields mechanically have to be the same length
    }

    /// Returns the TokenKind at the specified index, or nothing if out of bounds
    pub fn kind_at(&self, index: usize) -> Option<TokenKind> {
        self.kind.get(index).copied()
    }

    /// Returns a borrowed Literal at the specified index, or nothing if out of bounds
    pub fn literal_at(&self, index: usize) -> Option<&Literal> {
        self.literal.get(index)
    }

    /// Returns the token at an index's line number, or nothing if out of bounds
    pub fn line_at(&self, index: usize) -> Option<usize> {
        self.line.get(index).copied()
    }

    /// Returns the token at an index's line number, or nothing if out of bounds
    pub fn pos_at(&self, index: usize) -> Option<usize> {
        self.pos.get(index).copied()
    }

    /// Safely appends a new token to the SoA
    pub fn push(&mut self, kind: TokenKind, literal: Literal, line: usize, pos: usize) {
        self.kind.push(kind);
        self.literal.push(literal);
        self.line.push(line);
        self.pos.push(pos);
    }
}

#[cfg(test)]
mod keyword_matching_tests {
    use super::*;

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
        assert_eq!(
            KEYWORDS.get("instanceof").cloned(),
            Some(TokenKind::InstanceOf)
        )
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
        assert_eq!(
            KEYWORDS.get("protected").cloned(),
            Some(TokenKind::Protected)
        )
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
    fn true_keyword() {
        assert_eq!(KEYWORDS.get("true").cloned(), Some(TokenKind::Boolean))
    }

    #[test]
    fn false_keyword() {
        assert_eq!(KEYWORDS.get("false").cloned(), Some(TokenKind::Boolean))
    }

    #[test]
    fn no_match() {
        assert_eq!(KEYWORDS.get("This isn't a keyword").cloned(), None)
    }
}

#[cfg(test)]
mod token_pool_init_tests {
    use super::*;

    /// Verifies TokenPool::new() allocates specified capacity for all internal vectors.
    #[test]
    fn inits_correctly() {
        for n in 1..50 {
            let pool = TokenPool::new(n);

            assert_eq!(pool.len(), 0); // len != capacity
            assert_eq!(pool.kind.capacity(), n);
            assert_eq!(pool.line.capacity(), n);
            assert_eq!(pool.literal.capacity(), n);
            assert_eq!(pool.pos.capacity(), n);
        }
    }

    /// Verifies TokenPool::new(0) produces zero-capacity vectors.
    #[test]
    fn init_with_zero_capacity() {
        let pool = TokenPool::new(0);
        assert_eq!(pool.kind.capacity(), 0);
        assert_eq!(pool.literal.capacity(), 0);
        assert_eq!(pool.line.capacity(), 0);
        assert_eq!(pool.pos.capacity(), 0);
        assert_eq!(pool.len(), 0);
    }

    /// Verifies TokenPool::new(10000) pre-allocates large capacity efficiently.
    #[test]
    fn init_with_large_capacity() {
        let pool = TokenPool::new(10000);
        assert_eq!(pool.len(), 0);
        assert_eq!(pool.kind.capacity(), 10000);
        assert_eq!(pool.literal.capacity(), 10000);
        assert_eq!(pool.line.capacity(), 10000);
        assert_eq!(pool.pos.capacity(), 10000);
    }
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    /// Test Literal::Identifier variant
    #[test]
    fn literal_identifier() {
        let lit = Literal::Identifier("foo".to_string());
        assert_eq!(lit, Literal::Identifier("foo".to_string()));
        assert_ne!(lit, Literal::Identifier("bar".to_string()));
    }

    /// Test Literal::Number variant with integer-form text
    #[test]
    fn literal_number_integer() {
        let lit = Literal::Number("42".to_string());
        assert_eq!(lit, Literal::Number("42".to_string()));
        assert_ne!(lit, Literal::Number("0".to_string()));
    }

    /// Test Literal::Number variant with float-form text
    #[test]
    fn literal_number_float() {
        let lit = Literal::Number("3.14".to_string());
        assert_eq!(lit, Literal::Number("3.14".to_string()));
        assert_ne!(lit, Literal::Number("2.71".to_string()));
    }

    /// Test Literal::Number variant with large integer-form text
    #[test]
    fn literal_number_large_integer() {
        let lit = Literal::Number("9999999999".to_string());
        assert_eq!(lit, Literal::Number("9999999999".to_string()));
        assert_ne!(lit, Literal::Number("0".to_string()));
    }

    /// Test Literal::Number variant with high-precision float-form text
    #[test]
    fn literal_number_high_precision_float() {
        let lit = Literal::Number("2.718281828".to_string());
        assert_eq!(lit, Literal::Number("2.718281828".to_string()));
        assert_ne!(lit, Literal::Number("1.0".to_string()));
    }

    /// Test Literal::Boolean variant - true
    #[test]
    fn literal_boolean_true() {
        let lit = Literal::Boolean(true);
        assert_eq!(lit, Literal::Boolean(true));
        assert_ne!(lit, Literal::Boolean(false));
    }

    /// Test Literal::Boolean variant - false
    #[test]
    fn literal_boolean_false() {
        let lit = Literal::Boolean(false);
        assert_eq!(lit, Literal::Boolean(false));
        assert_ne!(lit, Literal::Boolean(true));
    }

    /// Test Literal::Char variant
    #[test]
    fn literal_char() {
        let lit = Literal::Char('x');
        assert_eq!(lit, Literal::Char('x'));
        assert_ne!(lit, Literal::Char('y'));
    }

    /// Test Literal::Str variant
    #[test]
    fn literal_str() {
        let lit = Literal::Str("hello".to_string());
        assert_eq!(lit, Literal::Str("hello".to_string()));
        assert_ne!(lit, Literal::Str("world".to_string()));
    }

    /// Test Literal::Object variant
    #[test]
    fn literal_object() {
        let lit1 = Literal::Object;
        let lit2 = Literal::Object;
        assert_eq!(lit1, lit2);
    }

    /// Test Literal::Array variant
    #[test]
    fn literal_array() {
        let lit1 = Literal::Array;
        let lit2 = Literal::Array;
        assert_eq!(lit1, lit2);
    }

    /// Test Literal::Dictionary variant
    #[test]
    fn literal_dictionary() {
        let lit1 = Literal::Dictionary;
        let lit2 = Literal::Dictionary;
        assert_eq!(lit1, lit2);
    }

    /// Test Literal::Nil variant
    #[test]
    fn literal_nil() {
        let lit1 = Literal::Nil;
        let lit2 = Literal::Nil;
        assert_eq!(lit1, lit2);
    }

    /// Test Literal::Nil is not equal to other variants
    #[test]
    fn literal_nil_not_equal_to_others() {
        let nil_lit = Literal::Nil;
        assert_ne!(nil_lit, Literal::Number("0".to_string()));
        assert_ne!(nil_lit, Literal::Str("".to_string()));
        assert_ne!(nil_lit, Literal::Boolean(false));
    }

    /// Test Literal equality - same type, same value
    #[test]
    fn literal_equality_same() {
        assert_eq!(
            Literal::Number("1".to_string()),
            Literal::Number("1".to_string())
        );
        assert_eq!(
            Literal::Number("1.0".to_string()),
            Literal::Number("1.0".to_string())
        );
        assert_eq!(
            Literal::Str("test".to_string()),
            Literal::Str("test".to_string())
        );
    }

    /// Test Literal equality - same type, different value
    #[test]
    fn literal_equality_different_value() {
        assert_ne!(
            Literal::Number("1".to_string()),
            Literal::Number("2".to_string())
        );
        assert_ne!(
            Literal::Number("1.0".to_string()),
            Literal::Number("2.0".to_string())
        );
        assert_ne!(Literal::Str("a".to_string()), Literal::Str("b".to_string()));
    }

    /// Test Literal equality - different variants are not equal even when text content matches
    #[test]
    fn literal_equality_different_type() {
        assert_ne!(Literal::Number("1".to_string()), Literal::Boolean(true));
        assert_ne!(
            Literal::Str("42".to_string()),
            Literal::Number("42".to_string())
        );
        assert_ne!(
            Literal::Identifier("x".to_string()),
            Literal::Str("x".to_string())
        );
        assert_ne!(Literal::Number("3".to_string()), Literal::Char('3'));
    }

    /// Test Literal clone behavior
    #[test]
    fn literal_clone() {
        let lit = Literal::Str("clone me".to_string());
        let cloned = lit.clone();
        assert_eq!(lit, cloned);
    }

    /// Test Literal::Number clone behavior — Number now stores a String, so cloning
    /// must produce an independent owned copy that compares equal to the original.
    #[test]
    fn literal_number_clone() {
        let lit = Literal::Number("123".to_string());
        let cloned = lit.clone();
        assert_eq!(lit, cloned);
    }
}

#[cfg(test)]
mod token_pool_push_tests {
    use super::*;

    /// Verifies push() increments pool.len() correctly after each token addition.
    #[test]
    fn push_increments_len() {
        let mut pool = TokenPool::new(10);
        assert_eq!(pool.len(), 0);

        pool.push(
            TokenKind::Identifier,
            Literal::Identifier("foo".to_string()),
            1,
            1,
        );
        assert_eq!(pool.len(), 1);

        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 5);
        assert_eq!(pool.len(), 2);
    }

    /// Tests push() with various TokenKind values to ensure different variants work.
    #[test]
    fn push_various_token_kinds() {
        let mut pool = TokenPool::new(10);

        pool.push(TokenKind::LeftParen, Literal::Nil, 1, 1);
        pool.push(TokenKind::RightParen, Literal::Nil, 1, 2);
        pool.push(TokenKind::Plus, Literal::Nil, 1, 3);
        pool.push(TokenKind::EqualEqual, Literal::Nil, 1, 4);
        pool.push(TokenKind::Identifier, Literal::Nil, 1, 5);

        assert_eq!(pool.len(), 5);
    }

    /// Tests push() with various Literal types (Identifier, Number, Str, Boolean).
    #[test]
    fn push_various_literal_types() {
        let mut pool = TokenPool::new(10);

        pool.push(
            TokenKind::Identifier,
            Literal::Identifier("x".to_string()),
            1,
            1,
        );
        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 2);
        pool.push(TokenKind::Number, Literal::Number("3.14".to_string()), 1, 3);
        pool.push(TokenKind::String, Literal::Str("hello".to_string()), 1, 4);
        pool.push(TokenKind::Boolean, Literal::Boolean(true), 1, 5);

        assert_eq!(pool.len(), 5);
    }

    /// Tests push() sequence producing correct final state for "var x = 42;".
    #[test]
    fn push_sequence_state() {
        let mut pool = TokenPool::new(5);

        pool.push(TokenKind::Var, Literal::Nil, 1, 1);
        pool.push(
            TokenKind::Identifier,
            Literal::Identifier("x".to_string()),
            1,
            5,
        );
        pool.push(TokenKind::Equal, Literal::Nil, 1, 7);
        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 9);
        pool.push(TokenKind::Semicolon, Literal::Nil, 1, 12);

        assert_eq!(pool.len(), 5);
    }

    /// Tests push() correctly tracks line numbers across multiple token insertions.
    #[test]
    fn push_line_tracking() {
        let mut pool = TokenPool::new(10);

        pool.push(TokenKind::Number, Literal::Number("1".to_string()), 1, 1);
        pool.push(TokenKind::Number, Literal::Number("2".to_string()), 2, 1);
        pool.push(TokenKind::Number, Literal::Number("3".to_string()), 3, 1);
        pool.push(TokenKind::Number, Literal::Number("4".to_string()), 10, 1);

        assert_eq!(pool.line_at(0), Some(1));
        assert_eq!(pool.line_at(1), Some(2));
        assert_eq!(pool.line_at(2), Some(3));
        assert_eq!(pool.line_at(3), Some(10));
    }
}

#[cfg(test)]
mod accessor_method_tests {
    use super::*;

    #[test]
    fn kind_at_returns_correct_kind() {
        let mut pool = TokenPool::new(5);
        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 1);
        pool.push(
            TokenKind::Identifier,
            Literal::Identifier("foo".to_string()),
            1,
            5,
        );
        pool.push(TokenKind::Plus, Literal::Nil, 1, 10);

        assert_eq!(pool.kind_at(0), Some(TokenKind::Number));
        assert_eq!(pool.kind_at(1), Some(TokenKind::Identifier));
        assert_eq!(pool.kind_at(2), Some(TokenKind::Plus));
    }

    #[test]
    fn literal_at_returns_correct_literal() {
        let mut pool = TokenPool::new(5);
        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 1);
        pool.push(
            TokenKind::Identifier,
            Literal::Identifier("foo".to_string()),
            1,
            5,
        );

        assert_eq!(pool.literal_at(0), Some(&Literal::Number("42".to_string())));
        assert_eq!(
            pool.literal_at(1),
            Some(&Literal::Identifier("foo".to_string()))
        );
    }

    #[test]
    fn line_at_returns_correct_line() {
        let mut pool = TokenPool::new(5);
        pool.push(TokenKind::Number, Literal::Number("1".to_string()), 5, 1);
        pool.push(TokenKind::Number, Literal::Number("2".to_string()), 10, 2);
        pool.push(TokenKind::Number, Literal::Number("3".to_string()), 15, 3);

        assert_eq!(pool.line_at(0), Some(5));
        assert_eq!(pool.line_at(1), Some(10));
        assert_eq!(pool.line_at(2), Some(15));
    }

    /// Verifies pos_at() returns the correct column position for each index.
    #[test]
    fn pos_at_returns_correct_position() {
        let mut pool = TokenPool::new(5);
        pool.push(TokenKind::Identifier, Literal::Nil, 1, 5);
        pool.push(TokenKind::Identifier, Literal::Nil, 1, 10);
        pool.push(TokenKind::Identifier, Literal::Nil, 1, 42);

        assert_eq!(pool.pos_at(0), Some(5));
        assert_eq!(pool.pos_at(1), Some(10));
        assert_eq!(pool.pos_at(2), Some(42));
    }
}

#[cfg(test)]
mod boundary_edge_tests {
    use super::*;

    /// Verifies accessor methods return None for indices beyond pool size.
    #[test]
    fn accessors_return_none_out_of_bounds() {
        let mut pool = TokenPool::new(10);
        pool.push(TokenKind::Identifier, Literal::Nil, 1, 1);

        assert_eq!(pool.kind_at(1), None);
        assert_eq!(pool.kind_at(100), None);
    }

    /// Verifies accessor methods return None when index equals current length.
    #[test]
    fn accessors_return_none_at_len() {
        let mut pool = TokenPool::new(10);
        pool.push(TokenKind::Identifier, Literal::Nil, 1, 1);

        assert_eq!(pool.kind_at(pool.len()), None);
    }

    /// Verifies accessor methods return None on empty pool.
    #[test]
    fn empty_pool_accessors() {
        let pool = TokenPool::new(5);

        assert_eq!(pool.kind_at(0), None);
        assert_eq!(pool.literal_at(0), None);
        assert_eq!(pool.line_at(0), None);
    }

    /// Verifies maximum valid index (len - 1) works, index = len returns None.
    #[test]
    fn maximum_valid_index() {
        let mut pool = TokenPool::new(10);
        for i in 0..5 {
            pool.push(TokenKind::Number, Literal::Number(i.to_string()), 1, i);
        }

        assert_eq!(pool.kind_at(4), Some(TokenKind::Number));
        assert_eq!(pool.kind_at(5), None);
    }

    /// Verifies len() returns 0 on newly constructed pool.
    #[test]
    fn empty_pool_len() {
        let pool = TokenPool::new(0);
        assert_eq!(pool.len(), 0);
    }

    /// Verifies accessor methods handle large indices on empty pool without panic.
    #[test]
    fn large_index_on_empty_pool() {
        let pool = TokenPool::new(5);

        assert_eq!(pool.kind_at(0), None);
        assert_eq!(pool.kind_at(1000), None);
        assert_eq!(pool.kind_at(usize::MAX), None);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Verifies push() then accessor read produces correct roundtrip data.
    #[test]
    fn push_read_roundtrip() {
        let mut pool = TokenPool::new(10);

        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 1);

        assert_eq!(pool.kind_at(0), Some(TokenKind::Number));
        assert_eq!(pool.literal_at(0), Some(&Literal::Number("42".to_string())));
        assert_eq!(pool.line_at(0), Some(1));
    }

    /// Tests full sequence "var x = 42;" with multiple token types.
    #[test]
    fn multiple_tokens_mixed() {
        let mut pool = TokenPool::new(10);

        pool.push(TokenKind::Var, Literal::Nil, 1, 1);
        pool.push(
            TokenKind::Identifier,
            Literal::Identifier("x".to_string()),
            1,
            5,
        );
        pool.push(TokenKind::Equal, Literal::Nil, 1, 7);
        pool.push(TokenKind::Number, Literal::Number("42".to_string()), 1, 9);
        pool.push(TokenKind::Semicolon, Literal::Nil, 1, 12);

        assert_eq!(pool.len(), 5);

        assert_eq!(pool.kind_at(0), Some(TokenKind::Var));
        assert_eq!(pool.kind_at(1), Some(TokenKind::Identifier));
        assert_eq!(pool.kind_at(2), Some(TokenKind::Equal));
        assert_eq!(pool.kind_at(3), Some(TokenKind::Number));
        assert_eq!(pool.kind_at(4), Some(TokenKind::Semicolon));
    }

    /// Verifies line numbers increment correctly across newlines in source.
    #[test]
    fn line_tracking_multiline() {
        let mut pool = TokenPool::new(10);

        pool.push(TokenKind::Number, Literal::Number("1".to_string()), 1, 1);
        pool.push(TokenKind::Plus, Literal::Nil, 2, 1);
        pool.push(TokenKind::Number, Literal::Number("2".to_string()), 2, 3);
        pool.push(TokenKind::Semicolon, Literal::Nil, 2, 5);

        assert_eq!(pool.line_at(0), Some(1));
        assert_eq!(pool.line_at(1), Some(2));
        assert_eq!(pool.line_at(2), Some(2));
        assert_eq!(pool.line_at(3), Some(2));
    }

    /// Verifies empty pool has zero length.
    #[test]
    fn empty_pool_verification() {
        let pool = TokenPool::new(0);
        assert_eq!(pool.len(), 0);
    }

    /// Verifies KEYWORDS map returns correct TokenKind for valid keywords.
    #[test]
    fn keyword_token_kind() {
        let keyword = KEYWORDS.get("var").cloned();
        assert_eq!(keyword, Some(TokenKind::Var));
    }

    /// Verifies KEYWORDS map returns None for non-keyword strings.
    #[test]
    fn non_keyword_returns_none() {
        let keyword = KEYWORDS.get("notakeyword").cloned();
        assert_eq!(keyword, None);
    }
}
