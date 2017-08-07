use super::{Ast, ParseError};

/// Parses a single AST.
pub fn one(c: &[char], i: &mut usize) -> Result<Ast, ParseError> {
    if *i >= c.len() {
        return Err(ParseError::UnexpectedEof(*i));
    }
    if c[*i] != '(' {
        return Err(ParseError::UnknownChar(*i, c[*i]));
    }
    *i += 1;
    let mut v = vec![];
    loop {
        if c[*i] == ')' {
            *i += 1;
            return Ok(Ast(v));
        } else {
            v.push(one(c, i)?);
        }
    }
}

#[test]
fn success() {
    let mut i = 0;
    let c = "(()(()))".chars().collect::<Vec<_>>();
    let r = one(&c, &mut i);
    assert_eq!(r, Ok(Ast(vec![
        Ast(vec![]),
        Ast(vec![
            Ast(vec![]),
        ]),
    ])));
}
