mod lexer;
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn inductive_test() {
        assert!(true)
    }

    #[test_case("Cargo.toml"; "verifying that Cargo.toml can be read")]
    fn sanity_check_fs(filename: &str) {
        let contents = lexer::read_file_to_str(filename);
        assert_ne!(contents, String::from("fail"))
    }
}

