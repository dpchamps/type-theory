lalrpop_mod!(pub parser, "/parser/parser.rs"); // synthesized by LALRPOP

use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

pub use crate::context::Context;
pub use crate::syntax::*;

pub fn parse(input: &str) -> Result<(Vec<Command>, Context), ParseError<usize, Token<'_>, &str>> {
    let mut context = Context::default();
    let result = parser::TopLevelParser::new().parse(&mut context, input)?;

    Ok((result, context))
}

#[cfg(test)]
mod tests {
    use crate::context::Context;
    use crate::syntax::*;
    use crate::*;
    use lalrpop_util::lexer::Token;
    use lalrpop_util::ParseError;

    #[test]
    fn test_parser_base() {
        let (commands, _) = parser::parse("").expect("");

        assert_eq!(commands, []);
    }

    #[test]
    fn test_parser_import() {
        let (commands, _) = parser::parse("use \"std\";").expect("");

        assert_eq!(commands, [Command::Import("std".into())]);
    }

    #[test]
    fn test_parser_multiple_import() {
        let (commands, _) = parser::parse(
            r#"
        use "std";
        use "bazinga";
        "#,
        )
        .expect("");

        assert_eq!(
            commands,
            [
                Command::Import("std".into()),
                Command::Import("bazinga".into())
            ]
        );
    }

    #[test]
    fn test_parser_garbage() {
        let mut context = Context::default();

        match parser::parse("ues \"std\";") {
            Err(ParseError::UnrecognizedToken { .. }) => {}
            Err(x) => panic!("Recieved unexpected result {:?}", x),
            Ok(x) => panic!("Recieved unexpected result {:?}", x),
        }
    }

    #[test]
    fn test_bind() {
        let (commands, context) = parser::parse("let x;").expect("");

        assert_eq!(
            commands,
            [Command::Bind(FileInfo::default(), Binding::NameBind)]
        );
        assert!(context.lookup_idx_by_name("x").is_ok());
    }

    #[test]
    fn test_bind_term() {
        let (commands, context) = parser::parse("let x = if true then false else true;").expect("");

        assert_eq!(
            commands,
            [Command::Bind(
                FileInfo::default(),
                Binding::TermBind(Box::new(Term::If(
                    FileInfo::default(),
                    Box::new(Term::True(FileInfo::default())),
                    Box::new(Term::False(FileInfo::default())),
                    Box::new(Term::True(FileInfo::default()))
                )))
            )]
        );
        // assert!(context.lookup_idx_by_name("x").is_ok());
    }

    #[test]
    fn test_lambda() {
        let expectation = [Command::Eval(
            FileInfo::default(),
            Term::Abstraction(
                FileInfo::default(),
                "x".into(),
                Box::new(Term::Var(FileInfo::default(), Var::default())),
            ),
        )];

        let input = r#"λ x.x;"#;
        let (commands, context) = parser::parse(input).unwrap();
        assert_eq!(commands, expectation);

        let input = r#"lambda x.x;"#;
        let (commands, context) = parser::parse(input).unwrap();
        assert_eq!(commands, expectation);
    }

    #[test]
    fn test_lambda_complex() {
        let expectation = [Command::Eval(
            FileInfo::default(),
            Term::Abstraction(
                FileInfo::default(),
                "x".into(),
                Box::new(Term::Abstraction(
                    FileInfo::default(),
                    "y".into(),
                    Box::new(Term::Application(
                        FileInfo::default(),
                        Box::new(Term::Var(FileInfo::default(), Var::default())),
                        Box::new(Term::Var(FileInfo::default(), Var::default())),
                    )),
                )),
            ),
        )];

        let input = r#"λ x. λ y. y x;"#;
        let (commands, context) = parser::parse(input).unwrap();
        assert_eq!(commands, expectation);
    }
}
