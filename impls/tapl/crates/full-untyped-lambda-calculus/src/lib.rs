#![feature(box_patterns)]
#![feature(box_syntax)]

mod syntax;
mod context;
mod evaluate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
