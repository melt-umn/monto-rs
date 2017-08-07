//! Useful data about the language of balanced parentheses, as a compiler
//! could expose it.

mod parse;

use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::FromStr;

/// The abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ast(pub Vec<Ast>);

impl Display for Ast {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_char('(')?;
        for ast in &self.0 {
            write!(fmt, "{}", ast)?
        }
        fmt.write_char(')')
    }
}

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
