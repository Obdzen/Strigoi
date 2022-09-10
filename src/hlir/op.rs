use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
}

impl Op {
    pub(crate) fn parser() -> impl Parser<Token, Op, Error = Simple<Token>> {
        choice((
            just(Token::Add).to(Op::Add),
            just(Token::Sub).to(Op::Sub),
            just(Token::Star).to(Op::Mul),
            just(Token::Div).to(Op::Div),
            just(Token::Percent).to(Op::Mod),
            just(Token::Ampresand).to(Op::And),
        ))
    }
}
