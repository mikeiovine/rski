use rski::Config;
use std::env;
use std::process;

fn show_usage() {
    eprintln!("usage: rski [script]");
}

fn main() {
    let config = Config::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("Config parsing failed with error:\n{}", err);
        show_usage();
        process::exit(1);
    });

    if let Err(err) = config.run() {
        eprintln!("Run failure with error:\n{}", err);
        process::exit(1);
    }
}
