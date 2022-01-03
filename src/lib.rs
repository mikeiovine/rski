mod parser;
use parser::Parser;
use std::fmt;

#[derive(Clone, Debug)]
enum Token {
    S,
    K,
    I,
    NestedTerm(CombinatoryTerm),
}

#[derive(Clone, Debug)]
pub struct CombinatoryTerm {
    // For efficiency reasons, tokens are stored in reverse order
    tokens: Vec<Token>,
}

impl CombinatoryTerm {
    pub fn new(token_seq: &str) -> Result<CombinatoryTerm, String> {
        let parser = Parser::new(token_seq);
        let tokens = parser.parse(false)?;
        Ok(CombinatoryTerm { tokens })
    }

    pub fn evaluate(&mut self) {
        if self.tokens.is_empty() {
            return;
        }

        let mut token = self.get_next_token();
        while self.apply_next_token(token) {
            token = self.get_next_token();
        }

        for token in &mut self.tokens {
            if let Token::NestedTerm(term) = token {
                term.evaluate();
            }
        }
    }

    fn apply_next_token(&mut self, token: Token) -> bool {
        match token {
            Token::S => self.evaluate_s(),
            Token::K => self.evaluate_k(),
            Token::I => self.evaluate_i(),
            Token::NestedTerm(mut inner_expr) => {
                self.tokens.append(&mut inner_expr.tokens);
                true
            }
        }
    }

    fn get_next_token(&mut self) -> Token {
        self.tokens.pop().unwrap()
    }

    fn evaluate_s(&mut self) -> bool {
        let num_tokens = self.tokens.len();
        if num_tokens < 3 {
            self.tokens.push(Token::S);
            return false;
        }

        let x = self.get_next_token();
        let y = self.get_next_token();
        let z = self.get_next_token();

        let inner_term = CombinatoryTerm {
            tokens: vec![z.clone(), y],
        };

        self.tokens.push(Token::NestedTerm(inner_term));
        self.tokens.push(z);
        self.tokens.push(x);
        true
    }

    fn evaluate_k(&mut self) -> bool {
        let num_tokens = self.tokens.len();
        if num_tokens < 2 {
            self.tokens.push(Token::K);
            return false;
        }
        let arg = self.tokens.pop().unwrap();
        self.tokens.pop();
        self.tokens.push(arg);
        true
    }

    fn evaluate_i(&mut self) -> bool {
        let num_tokens = self.tokens.len();
        if num_tokens < 1 {
            self.tokens.push(Token::I);
            return false;
        }
        true
    }
}

impl fmt::Display for CombinatoryTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tokens
            .iter()
            .rev()
            .map(|token| match token {
                Token::S => write!(f, "S"),
                Token::K => write!(f, "K"),
                Token::I => write!(f, "I"),
                Token::NestedTerm(term) => write!(f, "({})", term),
            })
            .collect::<Result<_, _>>()
    }
}
