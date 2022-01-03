use super::engine::Computation;
use std::cell::RefCell;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    S,
    K,
    I,
    NestedTerm(Computation),
}

impl Token {
    pub fn make_nested(tokens: Vec<Token>) -> Token {
        Token::NestedTerm(Computation::from_tokens(tokens))
    }
}

// Parses a combinator string into something that the
// computation engine can use.
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

    // Parse the given token seq into a string. Note that
    // the returned token sequence is backwards - this simplifies
    // the implementation of the computation engine, since we can
    // just use Vec<Token> and get the next term in O(1) time with pop().
    pub(super) fn parse(&self) -> Result<Vec<Token>, String> {
        self.parse_impl(/*until_closed_paren*/ false)
    }

    fn parse_impl(&self, until_closed_paren: bool) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];
        loop {
            let cur_char = self.token_seq.borrow_mut().pop();
            match cur_char {
                Some('S') => tokens.push(Token::S),
                Some('K') => tokens.push(Token::K),
                Some('I') => tokens.push(Token::I),
                Some('(') => {
                    let subexpr = self.parse_impl(true)?;
                    tokens.push(Token::make_nested(subexpr));
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

#[cfg(test)]
mod test {
    use super::*;

    fn test_parser(token_seq: &str, expected: &Vec<Token>) {
        let parser = Parser::new(token_seq);
        let actual = parser.parse().unwrap();
        assert_eq!(actual, *expected);
    }

    fn test_parser_fails(token_seq: &str) {
        let parser = Parser::new(token_seq);
        match parser.parse() {
            Ok(_) => panic!("Expected combinator {} to fail parsing", token_seq),
            _ => (),
        }
    }

    #[test]
    fn test_parser_no_nested() {
        let combinator = "SKSKI";
        let expected = vec![Token::I, Token::K, Token::S, Token::K, Token::S];
        test_parser(combinator, &expected);
    }

    #[test]
    fn test_parser_nested() {
        let combinator = "S(S(I(SK)S))(SK)";
        let expected = vec![
            Token::make_nested(vec![Token::K, Token::S]),
            Token::make_nested(vec![
                Token::make_nested(vec![
                    Token::S,
                    Token::make_nested(vec![Token::K, Token::S]),
                    Token::I,
                ]),
                Token::S,
            ]),
            Token::S,
        ];
        test_parser(combinator, &expected);
    }

    #[test]
    fn test_parser_lowercasee() {
        let combinator = "s(ks)i";
        let expected = vec![
            Token::I,
            Token::make_nested(vec![Token::S, Token::K]),
            Token::S,
        ];
        test_parser(combinator, &expected);
    }

    #[test]
    fn test_parser_fails_unrecognized_character() {
        test_parser_fails("s(sk)abc");
    }

    #[test]
    fn test_parser_fails_too_many_open_parens() {
        test_parser_fails("(((SKS");
    }

    #[test]
    fn test_parser_fails_not_enough_closed_parens() {
        test_parser_fails("((S)");
    }
}
