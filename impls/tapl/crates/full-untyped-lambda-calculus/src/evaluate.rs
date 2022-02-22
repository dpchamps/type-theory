use crate::context::*;
use crate::context_visitor::*;
use crate::parser::parse;
use crate::syntax::*;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use std::fs;

#[derive(Debug)]
pub enum EvalError {
    NoFile(String),
    Parse(String),
    EvalError(String),
}

fn is_numeric(t: &Term) -> bool {
    let x = match t {
        Term::Zero(_) => true,
        Term::Successor(_, box t1) => is_numeric(t1),
        _ => false,
    };

    x
}

fn is_value(t: &Term) -> bool {
    match t {
        Term::String(_, _) => true,
        Term::True(_) => true,
        Term::False(_) => true,
        _ if is_numeric(t) => true,
        Term::Abstraction(_, _, _) => true,
        Term::Float(_, _) => true,
        Term::Record(_, fields) => fields.iter().all(|(name, box term)| is_value(term)),
        _ => false,
    }
}

pub fn eval(file_name: &str) -> Result<(), EvalError> {
    let file = fs::read_to_string(file_name).map_err(|e| EvalError::NoFile(format!("{:?}", e)))?;
    let (commands, mut context) = parse(&file).map_err(|e| EvalError::Parse(format!("{:?}", e)))?;
    println!("{}", context);
    for command in commands {
        match command {
            Command::Import(s) => {
                println!("Module {}. Skipping...", s);
            }
            Command::Bind(_, name, bind) => {}
            Command::Eval(_, term) => {
                let term_with_global_context =
                    context
                        .into_iter()
                        .fold(
                            term.clone(),
                            |t, ContextMember { name, binding }| match binding {
                                Binding::TermBind(box bound_term) => Term::Let(
                                    FileInfo::default(),
                                    name.into(),
                                    box bound_term.clone(),
                                    box t,
                                ),
                                _ => t,
                            },
                        );
                let term_hydrated = hydrate_vars(&mut context, &term_with_global_context);
                let eval_term = evaluate_top(&mut context, &term_hydrated);

                println!("{}\n|\t-> {}", term, eval_term);
            }
        }
    }

    Ok(())
}

fn eval_inner(context: &Context, term: &Term) -> Option<Term> {
    match term {
        Term::Var(_, Var { name, index, .. }) => match context.get_binding(*index as usize) {
            Some(Binding::TermBind(box t)) => Some(t),
            _ => None,
        },

        Term::If(_, box Term::True(_), box t1, _) => Some(t1.clone()),
        Term::If(_, box Term::False(_), _, box t2) => Some(t2.clone()),
        Term::If(fi, box cond, box t1, box t2) => {
            let t_prime = eval_inner(context, cond)?;

            Some(Term::If(
                fi.clone(),
                box t_prime,
                box t1.clone(),
                box t2.clone(),
            ))
        }

        Term::Let(_, name, box v1, box t1) if is_value(v1) => Some(t1.substitute_top(v1)),
        Term::Let(file_info, name, box t1, box t2) => {
            let t_prime = eval_inner(context, t1)?;

            Some(Term::Let(
                file_info.clone(),
                name.clone(),
                box t_prime,
                box t2.clone(),
            ))
        }

        Term::Record(file_info, fields) => {
            unimplemented!("Record type evaluation not yet implemented")
        }
        Term::Projection(file_info, box Term::Record(_, fields), name) => {
            unimplemented!("Projection type evaluation not yet implemented")
        }
        Term::Projection(file_info, box t1, name) => {
            unimplemented!("Projection type evaluation not yet implemented")
        }

        Term::Application(file_info, box Term::Abstraction(_, name, t12), v2) if is_value(v2) => {
            Some(t12.substitute_top(v2))
        }
        Term::Application(file_info, v1, box t2) if is_value(v1) => {
            let t2_prime = eval_inner(context, t2)?;

            Some(Term::Application(
                file_info.clone(),
                v1.clone(),
                box t2_prime,
            ))
        }
        Term::Application(file_info, t1, t2) => {
            let t1_prime = eval_inner(context, t1)?;

            Some(Term::Application(
                file_info.clone(),
                box t1_prime,
                t2.clone(),
            ))
        }

        Term::Successor(file_info, box t1) => {
            let t1_prime = eval_inner(context, t1)?;

            Some(Term::Successor(file_info.clone(), box t1_prime))
        }

        Term::Predecessor(_, box Term::Zero(_)) => Some(Term::Zero(FileInfo::default())),
        Term::Predecessor(_, box Term::Successor(_, box nv_next)) if is_numeric(nv_next) => {
            Some(nv_next.clone())
        }
        Term::Predecessor(file_info, t1) => {
            let t1_prime = eval_inner(context, t1)?;

            Some(Term::Predecessor(file_info.clone(), box t1_prime))
        }

        Term::IsZero(_, box Term::Zero(_)) => Some(Term::True(FileInfo::default())),
        Term::IsZero(_, box Term::Successor(_, nv)) if is_numeric(nv) => {
            Some(Term::False(FileInfo::default()))
        }
        Term::IsZero(file_info, t) => {
            let t_prime = eval_inner(context, t)?;

            Some(Term::IsZero(file_info.clone(), box t_prime.clone()))
        }
        _ => None,
    }
}

fn hydrate_vars(context: &Context, term: &Term) -> Term {
    term.visit_with_context(context)
}

fn evaluate_top(context: &Context, term: &Term) -> Term {
    if let Some(t_prime) = eval_inner(context, &term) {
        return evaluate_top(context, &t_prime);
    }

    term.clone()
}

fn evaluate_binding(context: &mut Context, bind: &Binding) -> Binding {
    match bind {
        Binding::TermBind(box term) => {
            let term = hydrate_vars(context, &term);
            Binding::TermBind(box term)
        }
        _ => bind.clone(),
    }
}

#[cfg(test)]
mod tests {
    use crate::context::*;
    use crate::evaluate::*;
    use crate::parser::*;
    use crate::syntax::*;

    #[test]
    fn test_base_conditional() {
        let mut context = Context::default();
        let if_true = Term::If(
            FileInfo::default(),
            box Term::True(FileInfo::default()),
            box Term::Float(FileInfo::default(), 1.0),
            box Term::Float(FileInfo::default(), 9.0),
        );

        assert_eq!(
            evaluate_top(&mut context, &if_true),
            Term::Float(FileInfo::default(), 1.0)
        )
    }

    #[test]
    fn test_let() {
        let (parsed, mut context) = parse("let x = 1 in +x x;").expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let evaluated = evaluate_top(&mut context, term);
            let expectation = Term::Application(
                FileInfo::default(),
                box Term::from_int(2, FileInfo::default()),
                box Term::from_int(1, FileInfo::default()),
            );

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_let_free() {
        let (parsed, mut context) =
            parse("let y = true in let x = 1 in y x;").expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::Application(
                FileInfo::default(),
                box Term::True(FileInfo::default()),
                box Term::from_int(1, FileInfo::default()),
            );

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_let_global() {
        let (parsed, mut context) = parse("let y = true; let x = 1 in y x;").expect("Parse error");

        if let Command::Eval(_, term) = &parsed[1] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::Application(
                FileInfo::default(),
                box Term::True(FileInfo::default()),
                box Term::from_int(1, FileInfo::default()),
            );

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_lambda_evall() {
        let (parsed, mut context) = parse("(lambda x. x) 1;").expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::from_int(1, FileInfo::default());

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_advanced_one() {
        let input = r#"
        let tru = λt. λf. t in
        let fls = λt. λf. f in
        let realbool = λb.b true false in 
        realbool fls; 
        "#;

        let (parsed, mut context) = parse(input).expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::False(FileInfo::default());

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_advanced_two() {
        let input = r#"
        let czero = λs. λz. z in 
        let scc = λn. λs. λz. s (n s z) in
        let realnat = λm. m (λx. + x) 0 in
        realnat (scc czero); 
        "#;

        let (parsed, mut context) = parse(input).expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::from_int(1, FileInfo::default());

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_advanced_three() {
        let input = r#"
        let czero = λs. λz. z in 
        let scc = λn. λs. λz. s (n s z) in
        let Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y)) in
        let g = λfn. λn. if iszero n then czero else scc (fn (-n)) in
        let churchnat = Y g in
        let realnat = λm. m (λx. + x) 0 in
        realnat (churchnat 10);
        "#;

        let (parsed, mut context) = parse(input).expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::from_int(10, FileInfo::default());

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_advanced_four() {
        let input = r#"
        let czero = λs. λz. z in 
        let tru = λt. λf. t in
        let fls = λt. λf. f in
        let and = λb. λc. b c fls in
        let scc = λn. λs. λz. s (n s z) in
        let plus = λm. λn. λs. λz. m s (n s z) in
        let times = λm. λn. m (plus n) czero in
        let pair = λf. λs. λb. b f s in
        let fst = λp. p tru in
        let snd = λp. p fls in
        let iszro = λm. m (λx. fls) tru in
        let zz = pair czero czero in
        let ss = λp. pair (snd p) (plus (scc czero) (snd p)) in
        let prd = λm. fst (m ss zz) in
        let equal = λm. λn. and (iszro (m prd n)) (iszro (n prd m)) in 
        let Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y)) in
        let cn = λfn. λn. if iszero n then czero else scc (fn (-n)) in
        let churchnat = Y cn in
        let realeq = λm. λn. (equal m n) true false in
        let realnat = λm. m (λx. + x) 0 in
        let realbool = λb.b true false in 
        let fct = λfn. λn. if realeq n czero then (scc czero) else (times n (fn (prd n))) in
        let factorial = Y fct in
        realnat (factorial (churchnat 4));
        "#;

        let (parsed, mut context) = parse(input).expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::from_int(24, FileInfo::default());

            assert_eq!(evaluated, expectation);
        } else {
            panic!()
        }
    }
}
