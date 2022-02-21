#![allow(warnings)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#[macro_use]
extern crate lalrpop_util;

mod context;
mod evaluate;
mod parser;
mod syntax;
mod context_visitor;
mod printer;

fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
