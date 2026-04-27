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

use std::iter::Peekable;
use std::str::Chars;

use phf::phf_map;

use crate::{
    errors::ScanningError,
    tokens::{KEYWORDS, Literal, TokenKind, TokenPool},
};

fn scan_string(
    chars: &mut Peekable<Chars<'_>>,
    line: &mut usize,
    pos: &mut usize,
    start: usize,
) -> Result<(TokenKind, Literal), ScanningError> {
    let mut string_content = String::new();
    let mut terminated = false;
    let mut escaped = false;

    for string_char in chars.by_ref() {
        *pos += 1;
        if string_char == '\\' {
            escaped = true;
        }
        if string_char == '"' {
            if !escaped {
                terminated = true;
                break;
            } else {
                escaped = false;
            }
        }
        if string_char == '\n' {
            *line += 1;
            *pos = 0;
        }
        string_content.push(string_char);
    }

    if !terminated {
        return Err(ScanningError::UnterminatedString {
            line: *line,
            pos: start,
        });
    }

    Ok((TokenKind::String, Literal::Str(string_content)))
}

fn scan_number(
    first: char,
    chars: &mut Peekable<Chars<'_>>,
    line: usize,
    pos: &mut usize,
) -> Result<(TokenKind, Literal), ScanningError> {
    let mut number_text = String::from(first);
    let mut decimal_point_seen = false;

    while let Some(&next_char) = chars.peek() {
        if next_char.is_ascii_digit() {
            number_text.push(next_char);
            chars.next();
            *pos += 1;
        } else if next_char == '.' {
            if decimal_point_seen {
                return Err(ScanningError::UnexpectedCharacter {
                    line,
                    pos: *pos,
                    bad_char: next_char,
                });
            }
            // Only consume the dot if it's followed by a digit
            match chars.clone().nth(1) {
                Some(c) if c.is_ascii_digit() => {
                    decimal_point_seen = true;
                    number_text.push(next_char);
                    chars.next();
                    *pos += 1;
                }
                _ => {
                    // Dot is not part of this number; break and let main scanner handle it
                    break;
                }
            }
        } else {
            break;
        }
    }

    Ok((TokenKind::Number, Literal::Number(number_text)))
}

fn scan_identifier_or_keyword(
    first: char,
    chars: &mut Peekable<Chars<'_>>,
    pos: &mut usize,
) -> (TokenKind, Literal) {
    let mut identifier_text = String::from(first);

    while let Some(&next_char) = chars.peek() {
        if next_char.is_alphanumeric() {
            identifier_text.push(next_char);
            chars.next();
            *pos += 1;
        } else {
            break;
        }
    }

    let kind = KEYWORDS
        .get(&identifier_text)
        .copied()
        .unwrap_or(TokenKind::Identifier);

    let literal = if kind == TokenKind::Identifier {
        Literal::Identifier(identifier_text)
    } else {
        Literal::Nil
    };

    (kind, literal)
}

static SIMPLE_MATCHES: phf::Map<char, TokenKind> = phf_map!(
    '(' => TokenKind::LeftParen,
    ')' => TokenKind::RightParen,
    '{' => TokenKind::LeftBrace,
    '}' => TokenKind::RightBrace,
    '.' => TokenKind::Dot,
    ',' => TokenKind::Comma,
    '-' => TokenKind::Minus,
    '+' => TokenKind::Plus,
    '*' => TokenKind::Star,
    '%' => TokenKind::Percent,
    ';' => TokenKind::Semicolon,
    '~' => TokenKind::Tilde,
    '&' => TokenKind::Ampersand,
    '^' => TokenKind::Caret,
);

pub fn scan_tokens(source: &str) -> Result<TokenPool, ScanningError> {
    let mut tokens = TokenPool::new(source.len() / 4); // Guesstimate the amount of tokens, preallocate for performance

    let mut chars = source.chars().peekable();
    let mut line: usize = 1;
    let mut pos: usize = 0;

    while let Some(current_char) = chars.next() {
        pos += 1;

        match current_char {
            '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | '*' | '%' | '~' | '&' | '^' | ';' => {
                tokens.push(
                    *SIMPLE_MATCHES.get(&current_char).unwrap(), // safety: all entries that would trigger this branch are in the phf map
                    Literal::Nil,
                    line,
                    pos,
                )
            }

            '!' => {
                // token can be either ! or just !=.
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(TokenKind::BangEqual, Literal::Nil, line, pos);
                    pos += 1;
                } else {
                    tokens.push(TokenKind::Bang, Literal::Nil, line, pos);
                }
            }
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(TokenKind::EqualEqual, Literal::Nil, line, pos);
                    pos += 1;
                } else {
                    tokens.push(TokenKind::Equal, Literal::Nil, line, pos);
                }
            }
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(TokenKind::LessEqual, Literal::Nil, line, pos);
                    pos += 1;
                }
                Some('<') => {
                    chars.next();
                    tokens.push(TokenKind::DoubleLess, Literal::Nil, line, pos);
                    pos += 1;
                }
                _ => tokens.push(TokenKind::Less, Literal::Nil, line, pos),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(TokenKind::GreaterEqual, Literal::Nil, line, pos);
                    pos += 1;
                }
                Some('>') => {
                    chars.next();
                    tokens.push(TokenKind::DoubleGreater, Literal::Nil, line, pos);
                    pos += 1;
                }
                _ => tokens.push(TokenKind::Greater, Literal::Nil, line, pos),
            },
            '/' => {
                if chars.peek() == Some(&'/') {
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }

                        chars.next();
                        pos += 1;
                    }
                } else {
                    tokens.push(TokenKind::Slash, Literal::Nil, line, pos);
                }
            }

            '|' => {
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(TokenKind::DoubleVerticalBar, Literal::Nil, line, pos);
                    pos += 1;
                } else {
                    tokens.push(TokenKind::VerticalBar, Literal::Nil, line, pos);
                }
            }

            '?' => {
                if chars.peek() == Some(&':') {
                    chars.next();
                    pos += 1;
                    tokens.push(TokenKind::QuestionColon, Literal::Nil, line, pos);
                } else {
                    return Err(ScanningError::UnexpectedCharacter {
                        line,
                        pos,
                        bad_char: current_char,
                    });
                }
            }

            ' ' | '\r' | '\t' => continue,
            '\n' => {
                line += 1;
                pos = 0;
                continue;
            }

            '"' => {
                let start = pos;
                let (kind, literal) = scan_string(&mut chars, &mut line, &mut pos, start)?;
                tokens.push(kind, literal, line, pos);
            }

            _ if current_char.is_numeric() => {
                let (kind, literal) = scan_number(current_char, &mut chars, line, &mut pos)?;
                tokens.push(kind, literal, line, pos);
            }

            _ if current_char.is_ascii_alphabetic() || current_char == '_' => {
                let (kind, literal) =
                    scan_identifier_or_keyword(current_char, &mut chars, &mut pos);
                tokens.push(kind, literal, line, pos);
            }

            _ => {
                return Err(ScanningError::UnexpectedCharacter {
                    line,
                    pos,
                    bad_char: current_char,
                });
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod scanner_tests {
    use super::*;
    use crate::tokens::{Literal, TokenKind};

    fn scan_ok(source: &str) -> TokenPool {
        scan_tokens(source).unwrap()
    }

    /// Verifies empty source produces no tokens.
    #[test]
    fn empty_source_produces_empty_pool() {
        let tokens = scan_ok("");
        assert_eq!(tokens.len(), 0);
    }

    /// Verifies whitespace-only source produces no tokens.
    #[test]
    fn whitespace_only_produces_empty_pool() {
        let tokens = scan_ok("  \t\r");
        assert_eq!(tokens.len(), 0);
    }

    /// Verifies newline-only source produces no tokens.
    #[test]
    fn newlines_only_produce_empty_pool() {
        let tokens = scan_ok("\n\n\n");
        assert_eq!(tokens.len(), 0);
    }

    /// Verifies a single left parenthesis tokenizes correctly.
    #[test]
    fn single_left_paren() {
        let tokens = scan_ok("(");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::LeftParen));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single right parenthesis tokenizes correctly.
    #[test]
    fn single_right_paren() {
        let tokens = scan_ok(")");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::RightParen));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single left brace tokenizes correctly.
    #[test]
    fn single_left_brace() {
        let tokens = scan_ok("{");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::LeftBrace));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single comma tokenizes correctly.
    #[test]
    fn single_comma() {
        let tokens = scan_ok(",");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Comma));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single dot tokenizes correctly.
    #[test]
    fn single_dot() {
        let tokens = scan_ok(".");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Dot));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single minus tokenizes correctly.
    #[test]
    fn single_minus() {
        let tokens = scan_ok("-");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Minus));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single plus tokenizes correctly.
    #[test]
    fn single_plus() {
        let tokens = scan_ok("+");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Plus));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single semicolon tokenizes correctly.
    #[test]
    fn single_semicolon() {
        let tokens = scan_ok(";");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Semicolon));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single star tokenizes correctly.
    #[test]
    fn single_star() {
        let tokens = scan_ok("*");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Star));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single bang tokenizes correctly.
    #[test]
    fn single_bang() {
        let tokens = scan_ok("!");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Bang));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies slash tokenizes when not starting a comment.
    #[test]
    fn single_slash_not_comment() {
        let tokens = scan_ok("/x");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Slash));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single less-than tokenizes correctly.
    #[test]
    fn single_less() {
        let tokens = scan_ok("<");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Less));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single greater-than tokenizes correctly.
    #[test]
    fn single_greater() {
        let tokens = scan_ok(">");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Greater));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a single vertical bar tokenizes correctly.
    #[test]
    fn single_vertical_bar() {
        let tokens = scan_ok("|");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::VerticalBar));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies bang-equal tokenizes correctly.
    #[test]
    fn bang_equal() {
        let tokens = scan_ok("!=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::BangEqual));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies equal-equal tokenizes correctly.
    #[test]
    fn equal_equal() {
        let tokens = scan_ok("==");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::EqualEqual));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies less-equal tokenizes correctly.
    #[test]
    fn less_equal() {
        let tokens = scan_ok("<=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::LessEqual));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies double-less tokenizes correctly.
    #[test]
    fn double_less() {
        let tokens = scan_ok("<<");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::DoubleLess));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies greater-equal tokenizes correctly.
    #[test]
    fn greater_equal() {
        let tokens = scan_ok(">=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::GreaterEqual));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies double-greater tokenizes correctly.
    #[test]
    fn double_greater() {
        let tokens = scan_ok(">>");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::DoubleGreater));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies double-vertical-bar tokenizes correctly.
    #[test]
    fn double_vertical_bar() {
        let tokens = scan_ok("||");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::DoubleVerticalBar));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies a question and colon "?:" gets tokenized correctly.
    #[test]
    fn question_colon() {
        let tokens = scan_ok("?:");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::QuestionColon));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    #[test]
    fn lone_question_mark() {
        let tokens = scan_tokens("?");
        assert_eq!(
            tokens,
            Err(ScanningError::UnexpectedCharacter {
                line: 0,
                pos: 0,
                bad_char: '?',
            }),
        );
    }

    /// Verifies comments are skipped and line numbers advance.
    #[test]
    fn comment_skipped_then_next_line_token() {
        let tokens = scan_ok("// comment\n+");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Plus));
        assert_eq!(tokens.line_at(0), Some(2));
    }

    /// Verifies EOF comments produce no tokens.
    #[test]
    fn comment_at_eof_no_newline() {
        let tokens = scan_ok("// just a comment");
        assert_eq!(tokens.len(), 0);
    }

    /// Verifies unterminated strings return the expected error.
    #[test]
    fn unterminated_string_error() {
        match scan_tokens("\"hello") {
            Err(ScanningError::UnterminatedString { line, .. }) => assert_eq!(line, 1),
            Ok(tokens) => panic!("expected UnterminatedString, got Ok(len={})", tokens.len()),
            Err(other) => panic!("expected UnterminatedString, got {other:?}"),
        }
    }

    /// Verifies simple identifiers tokenize correctly.
    #[test]
    fn identifier_simple() {
        let tokens = scan_ok("foo");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Identifier));
        assert_eq!(tokens.line_at(0), Some(1));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Identifier("foo".to_string()))
        );
    }

    /// Verifies alphanumeric identifiers tokenize correctly.
    #[test]
    fn identifier_alphanumeric() {
        let tokens = scan_ok("foo123");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Identifier));
        assert_eq!(tokens.line_at(0), Some(1));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Identifier("foo123".to_string()))
        );
    }

    /// Verifies the var keyword is recognized.
    #[test]
    fn keyword_var_recognized() {
        let tokens = scan_ok("var");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Var));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies the if keyword is recognized.
    #[test]
    fn keyword_if_recognized() {
        let tokens = scan_ok("if");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::If));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies the while keyword is recognized.
    #[test]
    fn keyword_while_recognized() {
        let tokens = scan_ok("while");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::While));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies true tokenizes as a boolean kind.
    #[test]
    fn keyword_true_is_boolean_kind() {
        let tokens = scan_ok("true");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Boolean));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies false tokenizes as a boolean kind.
    #[test]
    fn keyword_false_is_boolean_kind() {
        let tokens = scan_ok("false");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Boolean));
        assert_eq!(tokens.line_at(0), Some(1));
    }

    /// Verifies integer numbers preserve their literal text.
    #[test]
    fn number_integer_literal() {
        let tokens = scan_ok("42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Number));
        assert_eq!(tokens.line_at(0), Some(1));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Number("42".to_string()))
        );
    }

    /// Verifies floating-point numbers preserve their literal text.
    #[test]
    fn number_float_literal() {
        let tokens = scan_ok("3.14");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Number));
        assert_eq!(tokens.line_at(0), Some(1));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Number("3.14".to_string()))
        );
    }

    /// Verifies double-decimal numbers return an unexpected-character error.
    #[test]
    fn number_two_decimals_is_error() {
        match scan_tokens("1.2.3") {
            Err(ScanningError::UnexpectedCharacter { bad_char, .. }) => assert_eq!(bad_char, '.'),
            Ok(tokens) => panic!("expected UnexpectedCharacter, got Ok(len={})", tokens.len()),
            Err(other) => panic!("expected UnexpectedCharacter, got {other:?}"),
        }
    }

    /// Verifies unexpected at-sign returns an error.
    #[test]
    fn unexpected_at_sign_error() {
        match scan_tokens("@") {
            Err(ScanningError::UnexpectedCharacter { bad_char, .. }) => assert_eq!(bad_char, '@'),
            Ok(tokens) => panic!("expected UnexpectedCharacter, got Ok(len={})", tokens.len()),
            Err(other) => panic!("expected UnexpectedCharacter, got {other:?}"),
        }
    }

    /// Verifies line numbers increment after newlines.
    #[test]
    fn line_increments_after_newline() {
        let tokens = scan_ok("+\n+");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.line_at(0), Some(1));
        assert_eq!(tokens.line_at(1), Some(2));
    }

    /// Verifies a simple var declaration produces expected kinds.
    #[test]
    fn var_decl_kinds_partial() {
        let tokens = scan_ok("var x;");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Var));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Identifier));
        assert_eq!(tokens.kind_at(2), Some(TokenKind::Semicolon));
    }

    /// Verifies multi-char and single-char operators can be adjacent.
    #[test]
    fn multi_then_single_back_to_back() {
        let tokens = scan_ok("==+");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::EqualEqual));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Plus));
    }

    /// Verifies comments can precede keywords on the next line.
    #[test]
    fn comment_then_keyword() {
        let tokens = scan_ok("// hello\nvar");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Var));
        assert_eq!(tokens.line_at(0), Some(2));
    }

    /// Verifies whitespace between tokens is ignored.
    #[test]
    fn whitespace_between_tokens() {
        let tokens = scan_ok("+ \t -");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Plus));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Minus));
    }

    /// Verifies unterminated multiline strings return errors without panicking.
    #[test]
    fn string_unterminated_with_internal_newline_does_not_panic() {
        match scan_tokens("\"line1\nline2") {
            Err(ScanningError::UnterminatedString { .. }) => {}
            Ok(tokens) => panic!("expected UnterminatedString, got Ok(len={})", tokens.len()),
            Err(other) => panic!("expected UnterminatedString, got {other:?}"),
        }
    }

    /// Verifies terminated strings emit a String token with the correct literal.
    #[test]
    fn string_literal_pushes_string_token() {
        let tokens = scan_ok("\"hello\"");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::String));
    }

    /// Verifies a single equals sign emits an Equal token.
    #[test]
    fn single_equal_emits_equal_token() {
        let tokens = scan_ok("=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Equal));
    }

    /// Verifies a right brace emits a RightBrace token.
    #[test]
    fn right_brace_emits_token() {
        let tokens = scan_ok("}");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::RightBrace));
    }

    /// Verifies identifiers starting with underscore are recognized.
    #[test]
    fn identifier_underscore_prefix_allowed() {
        let tokens = scan_ok("_foo");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Identifier));
    }

    /// Verifies a method-call dot should not be absorbed into a number.
    #[test]
    fn number_then_dot_method_call() {
        let tokens = scan_ok("42.foo");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Number));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Dot));
        assert_eq!(tokens.kind_at(2), Some(TokenKind::Identifier));
    }

    /// Verifies keywords carry Literal::Nil, not Literal::Identifier.
    #[test]
    fn keyword_carries_nil_literal() {
        let tokens = scan_ok("var");
        assert_eq!(tokens.literal_at(0), Some(&Literal::Nil));
    }

    /// Verifies a backslash inside a string sets the escaped flag, preventing the
    /// immediately following '"' from terminating the string early.
    #[test]
    fn string_escaped_quote_does_not_terminate() {
        // Source: "hello\"world" — the \" should not close the string.
        // The scanner should continue until the final unescaped '"'.
        let result = scan_tokens("\"hello\\\"world\"");
        // If the escaped-quote logic is broken the scanner terminates at \" and
        // then sees 'w' as a stray identifier, producing more than one token.
        // Regardless of what content the token carries, exactly one token should
        // be emitted and it must be TokenKind::String.
        match result {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 1, "expected exactly one String token");
                assert_eq!(tokens.kind_at(0), Some(TokenKind::String));
            }
            Err(e) => panic!("expected Ok, got error: {e:?}"),
        }
    }

    /// Verifies that a backslash alone (at end of string content before closing
    /// quote) sets escaped, so the closing '"' clears escaped rather than
    /// terminating — the string is then unterminated and must return an error.
    #[test]
    fn string_trailing_backslash_makes_unterminated() {
        // Source: "hello\" — backslash escapes the closing quote, so the string
        // is never terminated.
        match scan_tokens("\"hello\\\"") {
            Err(ScanningError::UnterminatedString { .. }) => {}
            Ok(tokens) => panic!(
                "expected UnterminatedString, got Ok with {} tokens",
                tokens.len()
            ),
            Err(other) => panic!("expected UnterminatedString, got {other:?}"),
        }
    }

    /// Verifies that a decimal point immediately followed by a non-digit letter
    /// produces separate tokens: Number, Dot, Identifier.
    #[test]
    fn number_dot_followed_by_letter_is_error() {
        let tokens = scan_ok("1.x");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Number));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Dot));
        assert_eq!(tokens.kind_at(2), Some(TokenKind::Identifier));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Number("1".to_string()))
        );
    }

    /// Verifies that a decimal point at end of input (no digit follows) produces
    /// separate Number and Dot tokens rather than an error.
    #[test]
    fn number_trailing_dot_at_eof_is_error() {
        // The dot is not part of the number if it's not followed by a digit,
        // so "1." should produce two tokens: Number("1") and Dot.
        let tokens = scan_ok("1.");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Number));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Dot));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Number("1".to_string()))
        );
    }

    /// Verifies the number loop exits cleanly when a non-digit non-dot character
    /// follows an integer, emitting two separate tokens.
    #[test]
    fn number_followed_by_operator_breaks_loop() {
        let tokens = scan_ok("42+");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.kind_at(0), Some(TokenKind::Number));
        assert_eq!(tokens.kind_at(1), Some(TokenKind::Plus));
        assert_eq!(
            tokens.literal_at(0),
            Some(&Literal::Number("42".to_string()))
        );
    }
}
