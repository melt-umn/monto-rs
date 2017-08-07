//! Useful data about the language of balanced parentheses, as a compiler
//! could expose it.

mod parse;

use std::str::FromStr;

/// The abstract syntax tree.
pub struct Ast(pub Vec<Ast>);

impl FromStr for Ast {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut idx = 0;
        let ast = parse::one(&chars, &mut idx)?;
        if idx == chars.len() {
            Ok(ast)
        } else {
            Err(ParseError::UnknownChar(idx, chars[idx]))
        }
    }
}

/// An error encountered while parsing.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    /// The end of input was encountered unexpectedly.
    UnexpectedEof(usize),

    /// A character that was neither `'('` nor `')'`.
    UnknownChar(usize, char),
}
