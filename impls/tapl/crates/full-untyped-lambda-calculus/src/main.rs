#![allow(warnings)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#[macro_use]
extern crate lalrpop_util;
use std::env;
use std::fs;
use std::path::PathBuf;

mod context;
mod context_visitor;
mod evaluate;
mod parser;
mod printer;
mod syntax;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::canonicalize(PathBuf::from(&args[1]))
        .unwrap()
        .into_os_string()
        .into_string()
        .expect("");
    println!("Reading {}", file);

    evaluate::eval(&file).expect("Failed to evaluate");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
