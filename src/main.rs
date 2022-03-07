use std::{env, process};

use minigrep_demo_fengzhyuan::Config;
use minigrep_demo_fengzhyuan::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Exception occured: {}", e);
        process::exit(1);
    }
}


