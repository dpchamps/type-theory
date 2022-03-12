#![feature(box_patterns)]
#![feature(box_syntax)]

use context::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum Type {
    Unknown,
    Arrow(Box<Type>, Box<Type>),
    Bool,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Arrow(box t1, box t2) => write!(f, "{} → {}", t1, t2),
            Type::Bool => write!(f, "BOOL"),
            Type::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum SimpleBinding {
    NameBind,
    VarBind(Type),
}

impl fmt::Display for SimpleBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleBinding::NameBind => write!(f, "NameBind"),
            SimpleBinding::VarBind(t) => write!(f, "VarBind({})", t),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Var {
    index: i32,
    container_size: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum SimpleTerm {
    Var(Var),
    Abstraction(String, Type, Box<SimpleTerm>),
    Application(Box<SimpleTerm>, Box<SimpleTerm>),
    True,
    False,
    Conditional(Box<SimpleTerm>, Box<SimpleTerm>, Box<SimpleTerm>),
}

impl Shift for SimpleBinding {}
impl Substitute for SimpleBinding {}
impl Binding for SimpleBinding {}

impl Shift for SimpleTerm {}
impl Substitute for SimpleTerm {}
impl Term for SimpleTerm {}

#[derive(Debug, PartialEq)]
enum TypeCheckError<B: Binding, T: Term> {
    IncorrectBinding(B),
    BindingNotFound,
    Fail(T, Type, Type),
}

trait TypeCheck<B: Binding> {
    fn get_type(ctx: &Context<B>, idx: &i32) -> Result<Type, TypeCheckError<B, Self>>
    where
        Self: Term;
    fn type_of(&self, ctx: &Context<B>) -> Result<Type, TypeCheckError<B, Self>>
    where
        Self: Term;
}

impl TypeCheck<SimpleBinding> for SimpleTerm {
    fn get_type(
        ctx: &Context<SimpleBinding>,
        idx: &i32,
    ) -> Result<Type, TypeCheckError<SimpleBinding, SimpleTerm>> {
        match ctx.get_binding(*idx as usize) {
            Some(SimpleBinding::VarBind(ty)) => Ok(ty.clone()),
            Some(incorrect_binding) => {
                Err(TypeCheckError::IncorrectBinding(incorrect_binding.clone()))
            }
            None => Err(TypeCheckError::BindingNotFound),
        }
    }

    fn type_of(
        &self,
        ctx: &Context<SimpleBinding>,
    ) -> Result<Type, TypeCheckError<SimpleBinding, SimpleTerm>> {
        match self {
            SimpleTerm::Var(Var { index, .. }) => SimpleTerm::get_type(ctx, index),
            SimpleTerm::Abstraction(param, tyT1, box t2) => {
                let name = ctx.get_free_name(param);
                let next_ctx = ctx.with_new_binding(&ContextMember {
                    name,
                    binding: SimpleBinding::VarBind(tyT1.clone()),
                });
                let tyT2 = t2.type_of(&next_ctx)?;

                Ok(Type::Arrow(box tyT1.clone(), box tyT2.clone()))
            }
            SimpleTerm::Application(box t1, box t2) => {
                let ty_t1 = t1.type_of(ctx)?;
                let ty_t2 = t2.type_of(ctx)?;

                match ty_t1 {
                    Type::Arrow(box ty_t11, box ty_t22) if ty_t11 == ty_t2 => Ok(ty_t22),
                    Type::Arrow(box ty_11, box ty_t22) => {
                        Err(TypeCheckError::Fail(self.clone(), ty_11, ty_t22))
                    }
                    _ => Err(TypeCheckError::Fail(
                        self.clone(),
                        Type::Arrow(box Type::Unknown, box Type::Unknown),
                        ty_t1,
                    )),
                }
            }
            SimpleTerm::True | SimpleTerm::False => Ok(Type::Bool),
            SimpleTerm::Conditional(box t1, box t2, box t3) => {
                let t_if = t1.type_of(ctx)?;
                if t_if != Type::Bool {
                    return Err(TypeCheckError::Fail(self.clone(), Type::Bool, t_if));
                };

                let t_t2 = t2.type_of(ctx)?;
                let t_t3 = t3.type_of(ctx)?;

                if t_t2 != t_t3 {
                    return Err(TypeCheckError::Fail(self.clone(), t_t2, t_t3));
                }

                Ok(t_t2)
            }
        }
    }
}

fn main() {
    let mut context = Context::<SimpleBinding>::default();
    // let type_checker = Typechecker{ context: &context};
}

#[cfg(test)]
mod test {
    use crate::*;
    use context::*;

    #[test]
    fn typecheck_bool() {
        let context = Context::<SimpleBinding>::default();
        assert_eq!(SimpleTerm::True.type_of(&context), Ok(Type::Bool));
        assert_eq!(SimpleTerm::False.type_of(&context), Ok(Type::Bool));
    }

    #[test]
    fn type_check_var() {
        let mut context = Context::<SimpleBinding>::default();
        let var = SimpleTerm::Var(Var {
            index: 0,
            container_size: 0,
        });

        context.append_binding(&ContextMember {
            name: "test".into(),
            binding: SimpleBinding::VarBind(Type::Bool),
        });

        assert_eq!(var.type_of(&context), Ok(Type::Bool));

        context.append_binding(&ContextMember {
            name: "somethingElse".into(),
            binding: SimpleBinding::NameBind,
        });

        assert_eq!(
            var.type_of(&context),
            Err(TypeCheckError::IncorrectBinding(SimpleBinding::NameBind))
        );

        let var = SimpleTerm::Var(Var {
            index: 10,
            container_size: 0,
        });

        assert_eq!(var.type_of(&context), Err(TypeCheckError::BindingNotFound));
    }

    #[test]
    fn type_check_conditional() {
        let mut context = Context::<SimpleBinding>::default();

        let conditional = SimpleTerm::Conditional(
            box SimpleTerm::True,
            box SimpleTerm::False,
            box SimpleTerm::True,
        );

        assert_eq!(conditional.type_of(&context), Ok(Type::Bool));

        let conditional = SimpleTerm::Conditional(
            box SimpleTerm::Abstraction("x".into(), Type::Bool, box SimpleTerm::False),
            box SimpleTerm::False,
            box SimpleTerm::True,
        );

        assert_eq!(
            conditional.type_of(&context),
            Err(TypeCheckError::Fail(
                conditional,
                Type::Bool,
                Type::Arrow(box Type::Bool, box Type::Bool)
            ))
        );

        let conditional = SimpleTerm::Conditional(
            box SimpleTerm::True,
            box SimpleTerm::False,
            box SimpleTerm::Abstraction("x".into(), Type::Bool, box SimpleTerm::False),
        );

        assert_eq!(
            conditional.type_of(&context),
            Err(TypeCheckError::Fail(
                conditional.clone(),
                Type::Bool,
                Type::Arrow(box Type::Bool, box Type::Bool)
            ))
        );
    }

    #[test]
    fn abstraction() {
        let mut context = Context::<SimpleBinding>::default();

        context.append_binding(&ContextMember {
            name: "yabadabadoo".into(),
            binding: SimpleBinding::VarBind(Type::Arrow(box Type::Bool, box Type::Unknown)),
        });

        let abstraction = SimpleTerm::Abstraction(
            "x".into(),
            Type::Bool,
            box SimpleTerm::Var(Var {
                index: 0,
                container_size: 0,
            }),
        );

        assert_eq!(
            abstraction.type_of(&context),
            Ok(Type::Arrow(
                box Type::Bool,
                box Type::Unknown
            ))
        )
    }
}
