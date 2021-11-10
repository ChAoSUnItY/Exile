mod gen;
mod lexer;
mod parser;

extern crate rayon;

use lexer::lex;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

fn main() {
    let file_name = "./examples/e1.exile";

    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        let line_vec = reader.lines().collect::<Vec<Result<String, Error>>>();

        if line_vec.par_iter().any(|result| result.is_err()) {
            eprintln!("Unable to read file {}", file_name);
            return;
        }

        let tokens = line_vec
            .par_iter()
            .map(|r| lex(r.as_ref().unwrap().to_string()))
            .flatten()
            .collect();

        // println!(
        //     "{:?}",
        //     tokens.lock().unwrap().to_vec()
        // );

        let methods = parser::parse(tokens);

        // println!(
        //     "{:?}",
        //     methods
        // );

        let llvm_code = gen::gen(methods);

        println!("{}", llvm_code);
    }
}
