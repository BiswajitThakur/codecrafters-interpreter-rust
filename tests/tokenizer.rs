use codecrafters_interpreter::{Lexer, Token};

#[test]
fn test_lexer_empty() {
    let input = "";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some((Token::Eof, 1, 0..0).into()));
    assert_eq!(lx.next(), None);
}

#[test]
fn test_lexer_brace() {
    let input = "())({}][()]";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some((Token::LeftParen, 1, 0..1).into()));
    assert_eq!(lx.next(), Some((Token::RightParen, 1, 1..2).into()));
    assert_eq!(lx.next(), Some((Token::RightParen, 1, 2..3).into()));
    assert_eq!(lx.next(), Some((Token::LeftParen, 1, 3..4).into()));
    assert_eq!(lx.next(), Some((Token::LeftBrace, 1, 4..5).into()));
    assert_eq!(lx.next(), Some((Token::RightBrace, 1, 5..6).into()));
    assert_eq!(lx.next(), Some((Token::RightBracket, 1, 6..7).into()));
    assert_eq!(lx.next(), Some((Token::LeftBracket, 1, 7..8).into()));
    assert_eq!(lx.next(), Some((Token::LeftParen, 1, 8..9).into()));
    assert_eq!(lx.next(), Some((Token::RightParen, 1, 9..10).into()));
    assert_eq!(lx.next(), Some((Token::RightBracket, 1, 10..11).into()));
    assert_eq!(lx.next(), Some((Token::Eof, 1, 10..10).into()));
    assert_eq!(lx.next(), None);
}

#[test]
fn test_lexer_single_or_two() {
    let input = "!!=>>=<=<+==</,-;*.*";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some((Token::Bang, 1, 0..1).into()));
    assert_eq!(lx.next(), Some((Token::BangEqual, 1, 1..3).into()));
    assert_eq!(lx.next(), Some((Token::Greater, 1, 3..4).into()));
    assert_eq!(lx.next(), Some((Token::GreaterEqual, 1, 4..6).into()));
    assert_eq!(lx.next(), Some((Token::LessEqual, 1, 6..8).into()));
    assert_eq!(lx.next(), Some((Token::Less, 1, 8..9).into()));
    assert_eq!(lx.next(), Some((Token::Plus, 1, 9..10).into()));
    assert_eq!(lx.next(), Some((Token::EqualEqual, 1, 10..12).into()));
    assert_eq!(lx.next(), Some((Token::Less, 1, 12..13).into()));
    assert_eq!(lx.next(), Some((Token::Slash, 1, 13..14).into()));
    assert_eq!(lx.next(), Some((Token::Comma, 1, 14..15).into()));
    assert_eq!(lx.next(), Some((Token::Minus, 1, 15..16).into()));
    assert_eq!(lx.next(), Some((Token::Semicolon, 1, 16..17).into()));
    assert_eq!(lx.next(), Some((Token::Star, 1, 17..18).into()));
    assert_eq!(lx.next(), Some((Token::Dot, 1, 18..19).into()));
    assert_eq!(lx.next(), Some((Token::Star, 1, 19..20).into()));
    assert_eq!(lx.next(), Some((Token::Eof, 1, 19..19).into()));
    assert_eq!(lx.next(), None);
}

#[test]
fn test_lexer_line() {
    let input = "++\n-==\n\n()\n*\n;;";
    let mut lx = Lexer::from(input);
    assert_eq!(lx.get_line(), 1);
    assert_eq!(lx.next(), Some((Token::Plus, 1, 0..1).into()));
    assert_eq!(lx.get_line(), 1);
    assert_eq!(lx.next(), Some((Token::Plus, 1, 1..2).into()));
    assert_eq!(lx.get_line(), 1);
    assert_eq!(lx.next(), Some((Token::Minus, 2, 3..4).into()));
    assert_eq!(lx.get_line(), 2);
    assert_eq!(lx.next(), Some((Token::EqualEqual, 2, 4..6).into()));
    assert_eq!(lx.get_line(), 2);
    assert_eq!(lx.next(), Some((Token::LeftParen, 4, 8..9).into()));
    assert_eq!(lx.get_line(), 4);
    assert_eq!(lx.next(), Some((Token::RightParen, 4, 9..10).into()));
    assert_eq!(lx.get_line(), 4);
    assert_eq!(lx.next(), Some((Token::Star, 5, 11..12).into()));
    assert_eq!(lx.get_line(), 5);
    assert_eq!(lx.next(), Some((Token::Semicolon, 6, 13..14).into()));
    assert_eq!(lx.get_line(), 6);
    assert_eq!(lx.next(), Some((Token::Semicolon, 6, 14..15).into()));
    assert_eq!(lx.get_line(), 6);
    assert_eq!(lx.next(), Some((Token::Eof, 6, 14..14).into()));
    assert_eq!(lx.get_line(), 6);
    assert_eq!(lx.next(), None);
    assert_eq!(lx.get_line(), 6);
    assert_eq!(lx.next(), None);
    assert_eq!(lx.get_line(), 6);
}

#[test]
fn test_lexer_string() {
    let input = r#""hello""#;
    let mut lx = Lexer::from(input);
    assert_eq!(
        lx.next(),
        Some((Token::String("hello".into()), 1, 0..7).into())
    );
    assert_eq!(lx.next(), Some((Token::Eof, 1, 6..6).into()));
    let input = r#"+"""hello // world""#;
    let mut lx = Lexer::from(input);
    assert_eq!(lx.next(), Some((Token::Plus, 1, 0..1).into()));
    assert_eq!(lx.next(), Some((Token::String("".into()), 1, 1..3).into()));
    assert_eq!(
        lx.next(),
        Some((Token::String("hello // world".into()), 1, 3..19).into())
    );
    assert_eq!(lx.next(), Some((Token::Eof, 1, 18..18).into()));
    assert_eq!(lx.next(), None);
}
