use std::{env::args, process::exit};

use mathexpression_parser::calculator::calculate;

fn main() {
    app()
}

fn app() {
    let input = args().skip(1).take(1).next().unwrap_or_else(|| {
        eprintln!("Argument is missing");
        exit(1)
    });

    match calculate(&input) {
        Ok(result) => {
            println!("{result}")
        }
        Err(_) => eprintln!("Error occured"),
    };
}
