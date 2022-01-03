mod engine;
mod parser;

use engine::CombinatoryTermImpl;
use std::cell::Cell;
use std::fmt;

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

impl fmt::Display for CombinatoryTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.term)
    }
}
