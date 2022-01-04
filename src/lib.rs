mod engine;
mod observer;
mod parser;

use engine::CombinatoryTermImpl;
pub use observer::*;
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

impl fmt::Display for CombinatoryTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.term)
    }
}
