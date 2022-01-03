mod engine;
mod parser;

use engine::CombinatoryTermImpl;
use parser::{Parser, Token};
use std::cell::Cell;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Computation {
    // For efficiency reasons, tokens are stored in reverse order
    tokens: Vec<Token>,
    owner: *const CombinatoryTermImpl,
}

pub struct CombinatoryTerm {
    term: Box<CombinatoryTermImpl>,
}

impl CombinatoryTerm {
    pub fn new(token_seq: &str) -> Result<CombinatoryTerm, String> {
        let mut term = Box::new(CombinatoryTermImpl::new(token_seq)?);
        term.set_owner();
        Ok(CombinatoryTerm { term })
    }

    pub fn evaluate(&mut self) {
        self.term.evaluate();
    }

    pub fn attach(&mut self, observer: Box<dyn Observer>) {
        self.term.attach(observer);
    }
}

#[derive(Clone, Copy)]
pub enum Signal {
    ComputationStart,
    ComputationStep,
    ComputationEnd,
}

pub trait Observer {
    fn notify(&self, term: &CombinatoryTermImpl, signal: Signal);
}

pub struct Printer {
    num_steps: Cell<u32>,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            num_steps: Cell::new(0),
        }
    }
}

impl Observer for Printer {
    fn notify(&self, term: &CombinatoryTermImpl, signal: Signal) {
        match signal {
            Signal::ComputationStart => println!("starting combinator: {}", term),
            Signal::ComputationEnd => {
                println!("derived {} after {} steps", term, self.num_steps.get())
            }
            Signal::ComputationStep => {
                println!("{}", term);
                self.num_steps.set(self.num_steps.get() + 1);
            }
        }
    }
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

    fn from_tokens(tokens: Vec<Token>) -> Computation {
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

impl fmt::Display for CombinatoryTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.term)
    }
}
