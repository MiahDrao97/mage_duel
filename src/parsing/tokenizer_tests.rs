#[cfg(test)]
mod tests {
    use crate::parsing::tokenizer::tokenize;
    use crate::parsing::tokens::{Token, Tokens};
    use test_case::test_case;


    #[test_case("" ; "Empty string")]
    #[test_case("        " ; "All whitespace")]
    #[test_case("        \n\t  \r" ; "All whitespace extended")]
    fn tokenize_white_space_input(script: &str) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();
        assert_eq!(vec.len(), 1);
        assert!(matches!(vec[0], Tokens::EOF));
    }

    #[test_case("2", 2 ; "One digit number")]
    #[test_case("234", 234 ; "Multi digit number")]
    fn tokenize_number(script: &str, expected_value: u16) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();

        assert_eq!(vec.len(), 2);

        if let Tokens::Numeric(number_token) = &vec[0] {
            assert_eq!(number_token.clone().get_value(), expected_value);
        } else {
            panic!("Expected to parse a numeric token.");
        }
        assert!(matches!(vec[1], Tokens::EOF));
    }
}