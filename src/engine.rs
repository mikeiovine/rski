use crate::observer::{Observer, Signal};
use crate::parser::{Parser, Token};
use std::fmt;

pub struct CombinatoryTermImpl {
    observers: Vec<Box<dyn Observer>>,
    computation: Computation,
}

impl CombinatoryTermImpl {
    pub fn new(token_seq: &str) -> Result<CombinatoryTermImpl, String> {
        let computation = Computation::new(token_seq)?;
        Ok(CombinatoryTermImpl {
            computation,
            observers: vec![],
        })
    }

    pub fn evaluate(&mut self) {
        self.notify_observers(Signal::ComputationStart);
        self.computation.evaluate();
        self.notify_observers(Signal::ComputationEnd);
    }

    pub fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn notify_observers(&self, signal: Signal) {
        for observer in &self.observers {
            observer.notify(&self, signal);
        }
    }

    pub fn set_owner(&mut self) {
        self.computation.set_owner(self);
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Computation {
    // For efficiency reasons, tokens are stored in reverse order
    tokens: Vec<Token>,
    owner: *const CombinatoryTermImpl,
}

impl Computation {
    pub fn new(token_seq: &str) -> Result<Computation, String> {
        let parser = Parser::new(token_seq);
        let tokens = parser.parse()?;
        Ok(Computation {
            tokens,
            owner: std::ptr::null(),
        })
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Computation {
        Computation {
            tokens,
            owner: std::ptr::null(),
        }
    }

    pub fn set_owner(&mut self, term: *const CombinatoryTermImpl) {
        self.owner = term;
        for token in &mut self.tokens {
            match token {
                Token::NestedTerm(computation) => computation.set_owner(term),
                _ => (),
            }
        }
    }

    pub fn notify_step(&self) {
        unsafe { (*self.owner).notify_observers(Signal::ComputationStep) };
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

        let inner_term = Computation {
            tokens: vec![z.clone(), y],
            owner: self.owner,
        };

        self.tokens.push(Token::NestedTerm(inner_term));
        self.tokens.push(z);
        self.tokens.push(x);
        self.notify_step();
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
        self.notify_step();
        true
    }

    fn evaluate_i(&mut self) -> bool {
        let num_tokens = self.tokens.len();
        if num_tokens < 1 {
            self.tokens.push(Token::I);
            return false;
        }
        self.notify_step();
        true
    }
}

impl fmt::Display for Computation {
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

impl fmt::Display for CombinatoryTermImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.computation)
    }
}
