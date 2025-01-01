use codecrafters_interpreter::{Lexer, Token, WithSpan};

#[test]
fn test_lexer_empty() {
    let input = "";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Eof, 0..0)));
    assert_eq!(lx.next(), None);
}

#[test]
fn test_lexer_brace() {
    let input = "())({}][()]";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftParen, 0..1)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::RightParen, 1..2)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::RightParen, 2..3)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftParen, 3..4)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftBrace, 4..5)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::RightBrace, 5..6)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::RightBracket, 6..7)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftBracket, 7..8)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::LeftParen, 8..9)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::RightParen, 9..10)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::RightBracket, 10..11)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Eof, 10..10)));
    assert_eq!(lx.next(), None);
}

#[test]
fn test_lexer_single_or_two() {
    let input = "!!=>>=<=<+==<";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Bang, 0..1)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::BangEqual, 1..3)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Greater, 3..4)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::GreaterEqual, 4..6)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::LessEqual, 6..8)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Less, 8..9)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Plus, 9..10)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::EqualEqual, 10..12)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Less, 12..13)));
    assert_eq!(lx.next(), Some(WithSpan::new(Token::Eof, 12..12)));
    assert_eq!(lx.next(), None);
}
