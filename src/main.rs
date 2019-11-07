extern crate math_parser;
use math_parser::eval;

use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        println!("{:?}", eval(input));
    }
}
