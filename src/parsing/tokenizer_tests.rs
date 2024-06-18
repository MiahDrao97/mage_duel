#[cfg(test)]
mod tests {
    use crate::game_zones::types::DamageType;
    use crate::parsing::tokenizer::tokenize;
    use crate::parsing::tokens::{DamageTypeToken, DiceToken, IntToken, StringToken, Token, Tokens};
    use test_case::test_case;
    use std::mem::discriminant;

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

    #[test_case("fire", DamageType::Fire ; "Parsing fire")]
    #[test_case("lightning", DamageType::Lightning ; "Parsing lightning")]
    #[test_case("ice", DamageType::Ice ; "Parsing ice")]
    #[test_case("psychic", DamageType::Psychic ; "Parsing psychic")]
    #[test_case("acid", DamageType::Acid ; "Parsing acid")]
    #[test_case("necrotic", DamageType::Necrotic ; "Parsing necrotic")]
    #[test_case("divine", DamageType::Divine ; "Parsing divine")]
    #[test_case("force", DamageType::Force ; "Parsing force")]
    fn tokenize_damage_type(script: &str, expected_value: DamageType) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();

        assert_eq!(vec.len(), 2);

        if let Tokens::DamageType(dmg_type_token) = &vec[0] {
            assert_eq!(discriminant(&dmg_type_token.clone().get_value()), discriminant(&expected_value));
        } else {
            panic!("Expected to parse a damage type token.");
        }
        assert!(matches!(vec[1], Tokens::EOF));
    }

    #[test_case("false", false ; "Parse false")]
    #[test_case("true", true ; "Parse true")]
    fn tokenize_boolean(script: &str, expected_value: bool) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();

        assert_eq!(vec.len(), 2);

        if let Tokens::Boolean(bool_token) = &vec[0] {
            assert_eq!(bool_token.clone().get_value(), expected_value);
        } else {
            panic!("Expected to parse a boolean token.");
        }
        assert!(matches!(vec[1], Tokens::EOF));
    }

    #[test_case("+ !", 3 ; "Parse plus then bang")]
    #[test_case("+!", 2 ; "Parse plus-bang")]
    #[test_case("==", 2 ; "Parse equals equals")]
    #[test_case("<=", 2 ; "Parse less than or equal to")]
    #[test_case(">=", 2 ; "Parse greater than or equal to")]
    #[test_case("=>", 2 ; "Parse arrow")]
    #[test_case("~=", 2 ; "Parse not equal to")]

    fn tokenize_syntax(script: &str, expected_token_count: usize) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();

        assert_eq!(vec.len(), expected_token_count);
        for i in 0 .. (expected_token_count-1) {
            assert!(matches!(vec[i], Tokens::Symbol(_)))
        }
        assert!(matches!(vec[expected_token_count - 1], Tokens::EOF));
    }

    #[test_case("in" ; "Parse in")]
    #[test_case("if" ; "Parse if")]
    #[test_case("else" ; "Parse else")]
    #[test_case("for" ; "Parse for")]
    #[test_case("from" ; "Parse from")]
    #[test_case("target" ; "Parse target")]
    #[test_case("func" ; "Parse func")]
    fn tokenize_keywords(script: &str) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();

        assert_eq!(vec.len(), 2);

        if let Tokens::Symbol(keyword_token) = &vec[0] {
            assert_eq!(keyword_token.clone().get_value(), String::from(script));
        } else {
            panic!("Expected to parse a keyword token.");
        }
        assert!(matches!(vec[1], Tokens::EOF));
    }

    #[test_case("// this is some stuff 1+1" ; "Parse comment")]
    #[test_case("// this is some stuff 1+1 \n" ; "Parse comment newline")]
    fn tokenize_comment(script: &str) {
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();
        assert_eq!(vec.len(), 1);
        assert!(matches!(vec[0], Tokens::EOF));
    }

    #[test]
    fn tokenize_real_thing() {
        let script = "\
            #attack
            [0]: {
                // firebolt
                $ = target(1 in Player);
                1d4 + 4 fire => $;
            }
            ";
        let result = tokenize(script);

        assert!(result.is_ok());
        let vec = result.unwrap();

        let expected = [
            Tokens::Symbol(StringToken::from("#")),
            Tokens::Identifier(StringToken::from("attack")),
            Tokens::Symbol(StringToken::from("[")),
            Tokens::Numeric(IntToken::try_from("0").unwrap()),
            Tokens::Symbol(StringToken::from("]")),
            Tokens::Symbol(StringToken::from(":")),
            Tokens::Symbol(StringToken::from("{")),
            Tokens::Identifier(StringToken::from("$")),
            Tokens::Symbol(StringToken::from("=")),
            Tokens::Symbol(StringToken::from("target")),
            Tokens::Symbol(StringToken::from("(")),
            Tokens::Numeric(IntToken::try_from("1").unwrap()),
            Tokens::Symbol(StringToken::from("in")),
            Tokens::Identifier(StringToken::from("Player")),
            Tokens::Symbol(StringToken::from(")")),
            Tokens::Symbol(StringToken::from(";")),
            Tokens::Numeric(IntToken::try_from("1").unwrap()),
            Tokens::Dice(DiceToken::try_from("d4").unwrap()),
            Tokens::Symbol(StringToken::from("+")),
            Tokens::Numeric(IntToken::try_from("4").unwrap()),
            Tokens::DamageType(DamageTypeToken::try_from("fire").unwrap()),
            Tokens::Symbol(StringToken::from("=>")),
            Tokens::Identifier(StringToken::from("$")),
            Tokens::Symbol(StringToken::from(";")),
            Tokens::Symbol(StringToken::from("}")),
            Tokens::EOF
        ];

        assert_eq!(vec.len(), expected.len());

        for i in 0 .. expected.len() {
            assert_eq!(vec[i], expected[i]);
        }
    }
}