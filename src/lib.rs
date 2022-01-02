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
    verbose: bool,
}

impl CombinatoryTerm {
    pub fn new(token_seq: &str, verbose: bool) -> Result<CombinatoryTerm, String> {
        let parser = Parser::new(token_seq, verbose);
        let tokens = parser.parse(false)?;
        Ok(CombinatoryTerm { tokens, verbose })
    }

    pub fn evaluate(&mut self) {
        if self.verbose {
            println!("{}", self);
        }
        self.evaluate_impl();
    }

    fn evaluate_impl(&mut self) {
        if !self.tokens.is_empty() {
            let token = self.tokens.pop().unwrap();
            self.apply_next_token(token);
        }
    }

    fn apply_next_token(&mut self, token: Token) {
        match token {
            Token::S => self.evaluate_s(),
            Token::K => self.evaluate_k(),
            Token::I => self.evaluate_i(),
            Token::NestedTerm(mut inner_expr) => inner_expr.evaluate_impl(),
        }
    }

    fn evaluate_s(&mut self) {
        let num_tokens = self.tokens.len();
        if num_tokens < 3 {
            self.tokens.push(Token::S);
            return;
        }

        let x = self.tokens.pop().unwrap();
        let y = self.tokens.pop().unwrap();
        let z = self.tokens.pop().unwrap();

        let inner_term = CombinatoryTerm {
            tokens: vec![z.clone(), y],
            verbose: self.verbose,
        };

        self.tokens.push(Token::NestedTerm(inner_term));
        self.tokens.push(z);
        self.tokens.push(x);
        self.evaluate()
    }

    fn evaluate_k(&mut self) {
        let num_tokens = self.tokens.len();
        if num_tokens < 2 {
            self.tokens.push(Token::K);
            return;
        }
        let arg = self.tokens.pop().unwrap();
        self.tokens.pop();
        self.tokens.push(arg);
        self.evaluate()
    }

    fn evaluate_i(&mut self) {
        let num_tokens = self.tokens.len();
        if num_tokens < 1 {
            self.tokens.push(Token::I);
            return;
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
