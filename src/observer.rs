use super::CombinatoryTermImpl;
use std::cell::Cell;

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
