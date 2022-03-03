#![feature(box_patterns)]
#![feature(box_syntax)]

use context::*;

#[derive(Clone, Debug, PartialEq)]
enum Type {
    Arrow(Box<Type>, Box<Type>),
    Bool
}

#[derive(Clone, Debug, PartialEq)]
enum SimpleBinding {
    NameBind,
    VarBind(Type)
}

#[derive(Clone, Debug, PartialEq)]
enum SimpleTerm {
    Var(i32, i32),
    Abstraction(String, Type, Box<SimpleTerm>),
    Application(Box<SimpleTerm>, Box<SimpleTerm>),
    True,
    False,
    Conditional(Box<SimpleTerm>, Box<SimpleTerm>, Box<SimpleTerm>)
}



impl Shift for SimpleBinding {}
impl Substitute for SimpleBinding{}
impl Binding for SimpleBinding{}

impl Shift for SimpleTerm {}
impl Substitute for SimpleTerm {}
impl Term for SimpleTerm {}

#[derive(Debug, PartialEq)]
enum TypeCheckError {
    IncorrectBinding(String),
    Fail(String)
}

trait TypeCheck<B: Binding> {
    fn get_type(ctx: &Context<B>, idx: &i32) ->Result<Type, TypeCheckError>;
    fn type_of(&self, ctx: &Context<B>) -> Result<Type, TypeCheckError>;
}

impl TypeCheck<SimpleBinding> for SimpleTerm {
    fn get_type(ctx: &Context<SimpleBinding>, idx: &i32) -> Result<Type, TypeCheckError> {
        match ctx.get_binding(*idx as usize) {
            Some(SimpleBinding::VarBind(ty)) => Ok(ty.clone()),
            _ => Err(TypeCheckError::IncorrectBinding(format!("")))
        }
    }

    fn type_of(&self, ctx: &Context<SimpleBinding>) -> Result<Type, TypeCheckError> {
        match self {
            SimpleTerm::Var(i, _) => SimpleTerm::get_type(ctx, i),
            SimpleTerm::Abstraction(param, tyT1, box t2) => {
                let name = ctx.get_free_name(param);
                let next_ctx = ctx.with_new_binding(&ContextMember{
                    name,
                    binding: SimpleBinding::VarBind(tyT1.clone())
                });
                let tyT2 = t2.type_of(&next_ctx)?;

                Ok(Type::Arrow(box tyT1.clone(), box tyT2.clone()))
            },
            SimpleTerm::Application(box t1, box t2) => {
                let ty_t1 = t1.type_of(ctx)?;
                let ty_t2 = t2.type_of(ctx)?;

                match ty_t1 {
                    Type::Arrow(box ty_t11, box ty_t22) if ty_t11 == ty_t2 => Ok(ty_t22),
                    Type::Arrow(_, _) => Err(TypeCheckError::Fail("Expected type application to have matching terms.".into())),
                    _ => Err(TypeCheckError::Fail("Expected arrow type for lhs of application.".into()))
                }
            },
            SimpleTerm::True | SimpleTerm::False => Ok(Type::Bool),
            SimpleTerm::Conditional(box t1, box t2, box t3) => {
                todo!()
            }
        }
    }
}

fn main() {
    let mut context = Context::<SimpleBinding>::default();
    // let type_checker = Typechecker{ context: &context};


}
