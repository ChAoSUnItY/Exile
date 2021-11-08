mod lexer;

extern crate rayon;

use lexer::{lex, Token};
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
    sync::{Arc, Mutex},
};

fn main() {
    let file_name = "./examples/e1.exile";

    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        let tokens = Arc::new(Mutex::new(Vec::<Token>::new()));
        let line_vec = reader.lines().collect::<Vec<Result<String, Error>>>();

        if line_vec.par_iter().any(|result| result.is_err()) {
            eprintln!("Unable to read file {}", file_name);
            return;
        }

        line_vec.par_iter().for_each(|r| {
            tokens
                .lock()
                .unwrap()
                .append(&mut lex(r.as_ref().unwrap().to_string()));
        });

        println!(
            "{:?}",
            tokens
        );
    }
}
