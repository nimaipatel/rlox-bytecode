#[cfg(test)]
mod tests {
    use crate::{scanner::scan, token::Token, token_type::TokenType};

    #[test]
    fn leftparen_rightparen_bang() {
        let source = "()!";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::LeftParen, b"(", 1),
            Token::new(TokenType::RightParen, b")", 1),
            Token::new(TokenType::Bang, b"!", 1),
            Token::new(TokenType::Eof, b"", 1),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn leftparen_rightparen_bangequal() {
        let source = "()!=";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::LeftParen, b"(", 1),
            Token::new(TokenType::RightParen, b")", 1),
            Token::new(TokenType::BangEqual, b"!=", 1),
            Token::new(TokenType::Eof, b"", 1),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn number_operator_number() {
        let source = "456!=123";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::Number, b"456", 1),
            Token::new(TokenType::BangEqual, b"!=", 1),
            Token::new(TokenType::Number, b"123", 1),
            Token::new(TokenType::Eof, b"", 1),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn something_then_comment_then_something() {
        let source = "456!=123// this is a comment\n789!=789";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::Number, b"456", 1),
            Token::new(TokenType::BangEqual, b"!=", 1),
            Token::new(TokenType::Number, b"123", 1),
            Token::new(TokenType::Number, b"789", 2),
            Token::new(TokenType::BangEqual, b"!=", 2),
            Token::new(TokenType::Number, b"789", 2),
            Token::new(TokenType::Eof, b"", 2),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn string_literal_nl_string_literal() {
        let source = "\"nice\"\n\"lol\"";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::String, b"\"nice\"", 1),
            Token::new(TokenType::String, b"\"lol\"", 2),
            Token::new(TokenType::Eof, b"", 2),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn identifier() {
        let source = "nice != 69";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::Identifier, b"nice", 1),
            Token::new(TokenType::BangEqual, b"!=", 1),
            Token::new(TokenType::Number, b"69", 1),
            Token::new(TokenType::Eof, b"", 1),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn test() {
        let source = "if nice == 69 {}";
        let act_out = scan(source.as_bytes()).unwrap();
        let exp_out = vec![
            Token::new(TokenType::If, b"if", 1),
            Token::new(TokenType::Identifier, b"nice", 1),
            Token::new(TokenType::EqualEqual, b"==", 1),
            Token::new(TokenType::Number, b"69", 1),
            Token::new(TokenType::LeftBrace, b"{", 1),
            Token::new(TokenType::RightBrace, b"}", 1),
            Token::new(TokenType::Eof, b"", 1),
        ];
        assert_eq!(act_out, exp_out);
    }

    #[test]
    fn test_decimal_token() {
        let source = "1.23 >= 3.45";
        let actual = scan(source.as_bytes()).unwrap();
        let expected = vec![
            Token::new(TokenType::Number, b"1.23", 1),
            Token::new(TokenType::GreaterEqual, b">=", 1),
            Token::new(TokenType::Number, b"3.45", 1),
            Token::new(TokenType::Eof, b"", 1),
        ];
        assert_eq!(actual, expected);
    }
}
