mod syntax;
mod context;
mod typechecker;

/// Crate to provide generic traits for various type checkers
/// 
/// 
pub use syntax::*;
pub use context::*;
pub use typechecker::*;


#[cfg(test)]
mod tests {
    use crate::syntax::*;
    use crate::context::*;
    use crate::typechecker::*;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestBinding {}

    impl Shift for TestBinding {
        fn shift_n(&self, d: i32, c: i32) -> TestBinding{
            unimplemented!()
        }
    }

    impl Substitute for TestBinding {
        fn substitute(&self, j: i32, s: &Self) -> TestBinding {
            unimplemented!()
        }
    }

    impl Binding for TestBinding {}

    #[test]
    fn it_appends_a_binding() {
        let mut context = Context::default();

        context.append_binding(&ContextMember {
            name: String::from("test"),
            binding: TestBinding::default()
        });

        assert_eq!(context.get_name(0).unwrap(), "test");
    }

    #[test]
    fn it_finds_a_binding(){
        let mut context = Context::default();

        context.append_binding(&ContextMember {
            name: String::from("test"),
            binding: TestBinding::default()
        });

        let (idx, ctx_member) = context.find(|(_, ContextMember{name, ..})| name == "test").unwrap();

        assert_eq!(idx, 0);
        assert_eq!(ctx_member.name, "test");
        assert_eq!(ctx_member.binding, TestBinding::default());
    }
}
