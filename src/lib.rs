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

    pub fn evaluate(mut self) -> CombinatoryTerm {
        if self.tokens.is_empty() {
            return self;
        }
        let token = self.tokens.pop().unwrap();
        self.apply_next_token(token)
    }

    fn apply_next_token(self, token: Token) -> CombinatoryTerm {
        match token {
            Token::S => self.evaluate_s(),
            Token::K => self.evaluate_k(),
            Token::I => self.evaluate_i(),
            Token::NestedTerm(inner_expr) => inner_expr.evaluate(),
        }
    }

    fn evaluate_s(mut self) -> CombinatoryTerm {
        let num_tokens = self.tokens.len();
        if num_tokens < 3 {
            self.tokens.push(Token::S);
            return self;
        }

        let x = self.tokens.pop().unwrap();
        let y = self.tokens.pop().unwrap();
        let z = self.tokens.pop().unwrap();

        let inner_term = CombinatoryTerm {
            tokens: vec![z.clone(), y],
        };

        self.tokens.push(Token::NestedTerm(inner_term));
        self.tokens.push(z);
        self.tokens.push(x);
        self.evaluate()
    }

    fn evaluate_k(mut self) -> CombinatoryTerm {
        let num_tokens = self.tokens.len();
        if num_tokens < 2 {
            self.tokens.push(Token::K);
            return self;
        }
        let arg = self.tokens.pop().unwrap();
        self.tokens.pop();
        self.tokens.push(arg);
        self.evaluate()
    }

    fn evaluate_i(mut self) -> CombinatoryTerm {
        let num_tokens = self.tokens.len();
        if num_tokens < 1 {
            self.tokens.push(Token::I);
            return self;
        }
        self.evaluate()
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
