mod engine;
mod observer;
mod parser;

use engine::CombinatoryTermImpl;
pub use observer::*;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;

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

fn repl_evaluate(combinator: String) -> Result<(), String> {
    let mut combinator = CombinatoryTerm::new(&combinator)?;
    combinator.evaluate();
    println!("{}", combinator);
    Ok(())
}

fn run_repl() -> Result<(), Box<dyn Error>> {
    let input = io::stdin();
    let mut output = io::stdout();
    loop {
        print!("> ");
        output.flush()?;
        let mut buffer = String::new();
        let n = input.read_line(&mut buffer)?;
        if n == 0 {
            break;
        }
        if let Err(err) = repl_evaluate(buffer) {
            eprintln!("{}", err);
        }
    }
    Ok(())
}

fn run_file(filename: &str) -> Result<(), Box<dyn Error>> {
    // TODO: Support more combinators in a single file
    let contents = fs::read_to_string(filename)?;
    let mut combinator = CombinatoryTerm::new(&contents)?;
    combinator.attach(Box::new(Printer::new()));
    combinator.evaluate();
    Ok(())
}

pub struct Config {
    filename: Option<String>,
}

impl Config {
    pub fn parse<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next().ok_or("not enough args")?;
        let filename = args.next();
        if let Some(_) = args.next() {
            return Err("too many args");
        }
        Ok(Config { filename })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.filename {
            None => run_repl()?,
            Some(filename) => run_file(&filename)?,
        }
        Ok(())
    }
}
