#[macro_use]
mod lexer;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    use super::lexer;
    use test_case::test_case;
    extern crate pest;
    #[test]
    fn inductive_test() {
        assert!(true)
    }

    #[test_case("Cargo.toml"; "verifying that Cargo.toml can be read")]
    fn sanity_check_fs(filename: &str) {
        let contents = lexer::read_file_to_str(filename);
        assert_ne!(contents, String::from("definitely does not match these characters"))
    }

    #[test_case("ex/hello_clash_world.clsh", "ex/hello_clash_world.pest"; "hello_clash_world_test")]
    fn hello_clash_world(clash_file: &str, pest_file: &str) {

        let path: &str = &"#!/bin/clash";
        let shebang: clast::Sebang = clast::Shebang::from_str_ref(path);

        let fn_ident: &str = &"main";
        let fn_arg_str: &str = &"";
        let fn_args = None;

        /*`clast::new_fn` takes four arguments:
         *
         *   - identity:   &str,
         *   - arguments:  Option<Arguments>,
         *   - return_sig: ReturnSignature,
         *   - scope:      Option<Scope>,
         *   - public:     bool
         */

        let mut main_fn: clast::Func = clast::new_fn(
            fn_ident, 
            Some(None),
            signature!(()),
            Some(None),
            true
        );

        let pfn_ident: &str = &"print";
        let pfn_arg_str: &str = &"Hello Clash World!";
        let pfn_arg: Option<clast::Arguments> = clast::Arguments::from_str_ref(pfn_arg_str);
        let print_fn: clast::BuiltinFunc = clast::new_builtin_fn_from_str_ref(pfn_ident, pfn_arg);
        
        &main_fn.scope.extend(vec![print_fn]);
        
        let comparison: clast::Clast = clast::from_vec(vec![main_fn]);
        assert_eq!(
            lexer::parse(clash_file, pest_file),
            comparison
        )
    }
}

