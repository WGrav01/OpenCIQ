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

use std::fmt;

// CODE SMELL: missing PartialEq prevents direct assert_eq! on errors in tests. Also missing std::error::Error impl — the standard idiomatic Rust error trait.
#[derive(Debug, Clone, PartialEq)]
pub enum ScanningError {
    UnexpectedCharacter {
        line: usize,
        pos: usize,
        bad_char: char,
    },
    UnterminatedString {
        line: usize,
        pos: usize,
    },
}

impl<'a> fmt::Display for ScanningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: mark the bad line
        match self {
            ScanningError::UnterminatedString { line, pos } => {
                write!(f, "Unterminated string starting at {line}:{pos}")
            }

            ScanningError::UnexpectedCharacter {
                line,
                pos,
                bad_char,
            } => {
                write!(f, "Unexpected character '{bad_char}' at {line}:{pos}")
            }
        }
    }
}

#[cfg(test)]
mod errors_tests {
    use super::*;

    /// Verifies Display output for UnterminatedString.
    #[test]
    fn display_unterminated_string() {
        let e = ScanningError::UnterminatedString { line: 5, pos: 12 };
        assert_eq!(format!("{e}"), "Unterminated string starting at 5:12");
    }

    /// Verifies Display output for UnexpectedCharacter.
    #[test]
    fn display_unexpected_character() {
        let e = ScanningError::UnexpectedCharacter {
            line: 1,
            pos: 4,
            bad_char: '@',
        };
        assert_eq!(format!("{e}"), "Unexpected character '@' at 1:4");
    }

    /// Verifies Debug formatting does not panic and includes the variant name.
    #[test]
    fn debug_includes_variant_name() {
        let e = ScanningError::UnterminatedString { line: 1, pos: 1 };
        let s = format!("{e:?}");
        assert!(s.contains("UnterminatedString"));
    }

    /// Verifies Clone produces an equivalent value (compared by Display since PartialEq is missing).
    #[test]
    fn clone_preserves_display_output() {
        let original = ScanningError::UnexpectedCharacter {
            line: 3,
            pos: 5,
            bad_char: '#',
        };
        let cloned = original.clone();
        assert_eq!(format!("{original}"), format!("{cloned}"));
    }

    /// Verifies Display handles whitespace bad_char (tab) without breaking format.
    #[test]
    fn display_unexpected_tab_character() {
        let e = ScanningError::UnexpectedCharacter {
            line: 1,
            pos: 1,
            bad_char: '\t',
        };
        let s = format!("{e}");
        assert!(s.contains("\t"));
        assert!(s.contains("at 1:1"));
    }

    /// Verifies position 0 (edge) renders correctly.
    #[test]
    fn display_position_zero() {
        let e = ScanningError::UnterminatedString { line: 0, pos: 0 };
        assert_eq!(format!("{e}"), "Unterminated string starting at 0:0");
    }
}
