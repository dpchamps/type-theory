use crate::syntax::*;
use crate::context::*;

pub trait VisitWithContext {
    fn visit_with_context(
        &self,
        global_context: &Context
    ) -> Self;
}

impl VisitWithContext for Term {
    fn visit_with_context(&self, global_context: &Context) -> Term
    {
        fn walk(context: &Context, container_size: i32, term: &Term) -> Term
        {
            match term {
                Term::String(_, _) => term.clone(),
                Term::Var(file_info, Var {name, ..}) => {
                    let index = context.lookup_idx_by_name(name).unwrap();
                    println!("Creating correct var csize: {}, index: {}, name: {}. ctx: {:#?}", container_size, index, name, context);
                    Term::Var(file_info.clone(), Var::new(name, index as i32, container_size))
                },
                Term::True(_) => term.clone(),
                Term::False(_) => term.clone(),
                Term::If(file_info, box t1, box t2, box t3) => Term::If(
                    file_info.clone(),
                    box walk(context, container_size, t1),
                    box walk(context, container_size, t2),
                    box walk(context, container_size, t3),
                ),
                Term::Let(file_info, name, box t1, box t2) => {
                    let ctx1 = context.add_name(&name);
                    Term::Let(
                        file_info.clone(),
                        name.clone(),
                        box walk(context, container_size, t1),
                        box walk(&ctx1, container_size + 1, t2),
                    )
                },
                Term::Projection(file_info, box t1, l) => {
                    Term::Projection(
                        file_info.clone(),
                        box walk(context, container_size, t1),
                        l.clone(),
                    )
                },
                Term::Record(file_info, fields) => Term::Record(
                    file_info.clone(),
                    fields
                        .iter()
                        .map(|(field_name, field_term)| {
                            (
                                String::from(field_name),
                                box walk(context, container_size, field_term),
                            )
                        })
                        .collect(),
                ),
                Term::Abstraction(file_info, name, box t2) => {
                    let ctx1 = context.add_name(name);
                        Term::Abstraction(
                            file_info.clone(),
                            name.clone(),
                            box walk(&ctx1, container_size + 1, t2),
                        )
                },
                Term::Application(file_info, box t1, box t2) => Term::Application(
                    file_info.clone(),
                    box walk(context, container_size, t1),
                    box walk(context, container_size, t2),
                ),
                Term::Zero(_) => term.clone(),
                Term::Successor(file_info, box t1) => {
                    Term::Successor(file_info.clone(), box walk(context, container_size, t1))
                }
                Term::Predecessor(file_info, box t1) => {
                    Term::Predecessor(file_info.clone(), box walk(context, container_size, t1))
                }
                Term::IsZero(file_info, box t1) => {
                    Term::IsZero(file_info.clone(), box walk(context, container_size, t1))
                }
                Term::Float(_, _) => term.clone(),
                Term::TimesFloat(file_info, box t1, box t2) => Term::TimesFloat(
                    file_info.clone(),
                    box walk(context, container_size, t1),
                    box walk(context, container_size, t2),
                ),
            }
        }

        walk(global_context, 0, &self)
    }
}