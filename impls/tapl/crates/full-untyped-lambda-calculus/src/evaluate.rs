use crate::context::*;
use crate::parser::parse;
use crate::syntax::*;
use crate::context_visitor::*;
use std::fs;
use lalrpop_util::ParseError;
use lalrpop_util::lexer::Token;


pub enum EvalError {
    NoFile(String),
    Parse(String),
    EvalError(String)
}

fn is_numeric(t: &Term) -> bool {
    match t {
        Term::Zero(_) => true,
        Term::Successor(_, box t1) => is_numeric(t1),
        _ => false    
    }
}

fn is_value(t: &Term) -> bool {
    match t {
        Term::String(_, _) => true,
        Term::True(_) => true,
        Term::False(_) => true,
        _ if is_numeric(t) => true,
        Term::Abstraction(_, _, _) => true,
        Term::Float(_, _) => true,
        Term::Record(_, fields) => {
            fields.iter().all(|(name, box term)| is_value(term))
        }
        _  => false
    }
}


pub fn eval(file_name: &str) -> Result<(), EvalError> {
    let file = fs::read_to_string(file_name).map_err(|e| EvalError::NoFile(format!("{:?}", e)))?;
    let (commands, mut context) = parse(&file).map_err(|e| EvalError::Parse(format!("{:?}", e)))?;

    for command in commands {
        match command {
            Command::Import(s) => {
                println!("Module {}. Skipping...", s);
            },
            Command::Bind(_, bind) => {
                let binding = evaluate_binding(&mut context, &bind);
            },
            Command::Eval(_, term) => {
                let term = evaluate_top(&mut context, &term);
            }
        }
    }

    Ok(())
}

fn eval_inner(context: &mut Context, term: &Term) -> Option<Term> {
    match term {
        Term::Var(_, Var {index, ..}) => {
            match context.get_binding(*index as usize){
                Some(Binding::TermBind(box t)) => Some(t),
                _ => None
            }
        },

        Term::If(_, box Term::True(_), box t1, _) => Some(t1.clone()),
        Term::If(_, box Term::False(_), _, box t2) => Some(t2.clone()),
        Term::If(fi, box cond, box t1, box t2) => {
            let t_prime = eval_inner(context, cond)?;

            Some(Term::If(fi.clone(), box t_prime, box t1.clone(), box t2.clone()))
        },

        Term::Let(_, name, box v1, box t1) if is_value(v1) => {
            println!("Substituting from let");
            Some(t1.substitute_top(v1))
        },
        Term::Let(file_info, name, box t1, box t2) => {
            context.append_name(name);

            let t_prime = eval_inner(context, t1)?;

            Some(Term::Let(file_info.clone(), name.clone(), box t_prime, box t2.clone()))
        }

        Term::Record(file_info, fields) => {
            unimplemented!("Record type evaluation not yet implemented")
        },
        Term::Projection(file_info, box Term::Record(_, fields), name) => {
            unimplemented!("Projection type evaluation not yet implemented")
        },
        Term::Projection(file_info, box t1, name) => {
            unimplemented!("Projection type evaluation not yet implemented")
        },

        Term::Application(file_info, box Term::Abstraction(_, name, t12), v2) if is_value(v2) => {
            println!("Substituting from application");
            Some(t12.substitute_top(v2))
        },
        Term::Application(file_info, v1, box t2) if is_value(v1) => {
            let t2_prime = eval_inner(context, t2)?;
           
            Some(Term::Application(file_info.clone(), v1.clone(), box t2_prime))
        },
        Term::Application(file_info, t1, t2) => {
            let t1_prime = eval_inner(context, t1)?;

            Some(Term::Application(file_info.clone(), box t1_prime, t2.clone()))
        },

        Term::Successor(file_info, box t1) => {
            let t1_prime = eval_inner(context, t1)?;

            Some(Term::Successor(file_info.clone(), box t1_prime))
        },

        Term::Predecessor(_, box Term::Zero(_)) => {
            Some(Term::Zero(FileInfo::default()))
        },
        Term::Predecessor(_, box Term::Successor(_, box nv_next)) if is_numeric(nv_next) => {
            Some(nv_next.clone())
        },
        Term::Predecessor(file_info, t1) => {
            let t1_prime = eval_inner(context, t1)?;

            Some(Term::Predecessor(file_info.clone(), box t1_prime))
        }
        _ => None
    }
}

fn hydrate_vars(context: &mut Context, term: &Term) -> Term {
    term.visit_with_context(context)
}

fn evaluate_top(context: &mut Context, term: &Term) -> Term {
    println!("Evaluating {:#?}", term);
    
    if let Some(t_prime) = eval_inner(context, &term) {
        // println!("\t|> Evaluated to {:?}", t_prime);
        return evaluate_top(context, &t_prime)
    }

    println!("{:#?}", term);
    term.clone()
}

fn evaluate_binding(context: &mut Context, bind:  &Binding) -> Binding {
    match bind {
        Binding::TermBind(box term) => Binding::TermBind(box evaluate_top(context, &term)),
        _ => bind.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::context::*;
    use crate::syntax::*;
    use crate::evaluate::*;
    use crate::parser::*;


    #[test]
    fn test_base_conditional(){
        let mut context = Context::default();
        let if_true = Term::If(
            FileInfo::default(),
            box Term::True(FileInfo::default()),
            box Term::Float(FileInfo::default(), 1.0),
            box Term::Float(FileInfo::default(), 9.0)
        );

        assert_eq!(
            evaluate_top(&mut context, &if_true),
            Term::Float(FileInfo::default(), 1.0)
        )
    }

    #[test]
    fn test_let(){
        let (parsed, mut context) = parse("let x = 1 in +x x;").expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let evaluated = evaluate_top(&mut context, term);
            let expectation = Term::Application(
                FileInfo::default(),
                box Term::from_int(2, FileInfo::default()),
                box Term::from_int(1, FileInfo::default())
            );

            assert_eq!(
                evaluated,
                expectation
            );
        }else {
            panic!()
        }

        // panic!();
    }

    #[test]
    fn test_let_free(){
        let (parsed, mut context) = parse("let y = true in let x = 1 in y x;").expect("Parse error");

        if let Command::Eval(_, term) = &parsed[0] {
            let term = hydrate_vars(&mut context, term);
            println!("{:#?}", term);
            let evaluated = evaluate_top(&mut context, &term);
            let expectation = Term::Application(
                FileInfo::default(),
                box Term::True(FileInfo::default()),
                box Term::from_int(1, FileInfo::default())
            );

            assert_eq!(
                evaluated,
                expectation
            );
        }else {
            panic!()
        }
    }
}