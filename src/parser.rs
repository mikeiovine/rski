use crate::{CombinatoryTerm, Token};
use std::cell::RefCell;

pub struct Parser {
    token_seq: RefCell<Vec<char>>,
}

impl Parser {
    fn prepare_string_for_parsing(token_seq: &str) -> Vec<char> {
        token_seq.to_uppercase().chars().rev().collect()
    }

    fn make_mismatched_error<T>() -> Result<T, String> {
        // TODO should have better error message here.
        Err("Mismatched parentheses in expression".to_string())
    }

    pub fn new(token_seq: &str) -> Parser {
        let token_seq = Parser::prepare_string_for_parsing(token_seq);
        Parser {
            token_seq: RefCell::new(token_seq),
        }
    }

    pub(super) fn parse(&self, until_closed_paren: bool) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];
        loop {
            let cur_char = self.token_seq.borrow_mut().pop();
            match cur_char {
                Some('S') => tokens.push(Token::S),
                Some('K') => tokens.push(Token::K),
                Some('I') => tokens.push(Token::I),
                Some('(') => {
                    let subexpr = self.parse(true)?;
                    tokens.push(Token::NestedTerm(CombinatoryTerm { tokens: subexpr }));
                }
                Some(')') => {
                    if !until_closed_paren {
                        return Parser::make_mismatched_error();
                    }
                    return Ok(tokens.into_iter().rev().collect());
                }
                Some(c) => return Err(format!("Unexpected token: {}", c)),
                None => break,
            }
        }

        if until_closed_paren {
            return Parser::make_mismatched_error();
        } else {
            debug_assert!(self.token_seq.borrow().is_empty());
        }

        Ok(tokens.into_iter().rev().collect())
    }
}
