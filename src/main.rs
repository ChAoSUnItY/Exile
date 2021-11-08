use std::{fs::File, io::{self, BufRead, BufReader}};

mod lexer;

fn main() {
    if let Ok(file) = File::open("./examples/e1.exile") {
        let reader = BufReader::new(file);

        for result in reader.lines() {
            if let Ok(line) = result {
                println!("{:?}", lexer::lex(line));
            }
        }
    }
}
