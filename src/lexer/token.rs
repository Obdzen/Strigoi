use dbg_pls::DebugPls;
use std::fmt;
use std::ops::Range;

use chumsky::prelude::*;

#[derive(Debug, DebugPls, PartialEq, Clone, Hash, Eq)]
pub enum Token {
    // Keywords
    Pub,
    Fn,
    Extern,
    Let,
    Const,
    Return,
    // Sigiils
    LParan,
    RParan,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Colon,
    SemiColon,
    Comma,
    Period,
    At,
    Pound,
    Add,
    Sub,
    Star,
    Div,
    Equal,
    Ampresand,
    Carrot,
    Percent,
    DollarSign,
    Exclamation,
    GreaterThan,
    LessThan,

    True,
    False,
    Ident(String),
    Num(i64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Fn => write!(f, "fn"),
            Token::Pub => write!(f, "pub"),
            Token::Div => write!(f, "/"),
            Token::Equal => write!(f, "="),
            Token::Extern => write!(f, "extern"),
            Token::Let => write!(f, "let"),
            Token::Const => write!(f, "const"),
            Token::Return => write!(f, "return"),
            Token::LParan => write!(f, "("),
            Token::RParan => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Period => write!(f, "."),
            Token::Colon => write!(f, ":"),
            Token::SemiColon => write!(f, ";"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Ampresand => write!(f, "&"),
            Token::Carrot => write!(f, "^"),
            Token::Percent => write!(f, "%"),
            Token::DollarSign => write!(f, "$"),
            Token::Pound => write!(f, "#"),
            Token::At => write!(f, "@"),
            Token::Exclamation => write!(f, "!"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThan => write!(f, "<"),

            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Ident(data) => write!(f, "{}", data),
            Token::Num(data) => write!(f, "{}", data),
        }
    }
}

impl Token {
    pub(crate) fn lexer() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
        let keywords = choice((
            just("pub").to(Token::Pub),
            just("extern").to(Token::Extern),
            just("fn").to(Token::Fn),
            just("let").to(Token::Let),
            just("const").to(Token::Const),
            just("true").to(Token::True),
            just("false").to(Token::False),
            just("return").to(Token::Return),
        ));
        let sigils = choice((
            just("(").to(Token::LParan),
            just(")").to(Token::RParan),
            just("{").to(Token::LCurly),
            just("}").to(Token::RCurly),
            just("[").to(Token::LBracket),
            just("]").to(Token::RBracket),
            just(".").to(Token::Period),
            just('%').to(Token::Percent),
            just(",").to(Token::Comma),
            just(";").to(Token::SemiColon),
            just(":").to(Token::Colon),
            just("@").to(Token::At),
            just("^").to(Token::Carrot),
            just("#").to(Token::Pound),
            just("$").to(Token::DollarSign),
            just("!").to(Token::Exclamation),
            just("+").to(Token::Add),
            just("-").to(Token::Sub),
            just("*").to(Token::Star),
            just("/").then_ignore(none_of("/")).to(Token::Div),
            just("=").to(Token::Equal),
            just(">").to(Token::GreaterThan),
            just("<").to(Token::LessThan),
        ));
        let token = choice((
            sigils,
            keywords,
            text::ident().map(Token::Ident),
            text::digits(10).from_str().unwrapped().map(Token::Num),
        ));

        let comment = just("//").then(take_until(just('\n'))).padded();

        token
            .map_with_span(|token, span| (token, span))
            .padded_by(comment.repeated())
            .padded()
            .repeated()
            .then_ignore(end())
    }
}
