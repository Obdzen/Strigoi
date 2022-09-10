use crate::lexer::Token;
use chumsky::prelude::*;
use dbg_pls::DebugPls;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Ident(String),
    Bool(bool),
}

impl Value {
    pub fn parser() -> impl Parser<Token, Value, Error = Simple<Token>> {
        let num = select! { Token::Num(data) => data.clone() };
        let ident = select! { Token::Ident(data) => data.clone() };
        choice((
            num.map(Value::Int),
            ident.map(Value::Ident),
            just(Token::True).to(Value::Bool(true)),
            just(Token::False).to(Value::Bool(false)),
        ))
    }
}
