#[macro_use]
extern crate lazy_static;
extern crate clap;

use std::{fs, io::prelude::*, rc::Rc};

use lalr1plus::parser::*;

mod attributes;
mod bootstrap;
mod grammar;
mod symbols;

fn main() {
    let matches = clap::App::new("Augmented Lexical Analyzer and Parser Generator")
        .arg(clap::Arg::with_name("input").required(true))
        .get_matches();
    let file_name = matches
        .value_of("input")
        .expect("\"input\" is a required argument");
    let mut file = fs::File::open(file_name).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut grammar_specification = grammar::GrammarSpecification::new();
    if let Err(error) = grammar_specification.parse_text(input, file_name.to_string()) {
        writeln!(std::io::stderr(), "Parse failed: {:?}", error).unwrap();
        std::process::exit(1);
    }

    for symbol in grammar_specification.symbol_table.unused_symbols() {
        let location = symbol.defined_at().unwrap();
        grammar::report_warning(
            &location,
            &format!("Symbol \"{}\" is not used", symbol.name()),
        );
    }

    let mut undefined_symbols = 0;
    for symbol in grammar_specification.symbol_table.undefined_symbols() {
        for location in symbol.used_at() {
            grammar::report_error(
                &location,
                &format!("Symbol \"{}\" is not defined", symbol.name()),
            );
        }
        undefined_symbols += 1;
    }

    if (undefined_symbols + grammar_specification.error_count) > 0 {
        writeln!(
            std::io::stderr(),
            "Too man errors {} aborting.",
            (undefined_symbols + grammar_specification.error_count)
        )
        .unwrap();
        std::process::exit(2);
    }
}
