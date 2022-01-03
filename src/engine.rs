use super::{Computation, Observer, Signal};
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

impl fmt::Display for CombinatoryTermImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.computation)
    }
}
