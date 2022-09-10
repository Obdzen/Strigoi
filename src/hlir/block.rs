use super::{Spanned, Stmt};
use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct Block(pub Vec<Spanned<Stmt>>);

impl Block {
    pub(crate) fn parser() -> impl Parser<Token, Block, Error = Simple<Token>> {
        Stmt::parser()
            .map_with_span(|stmt, span| (span, stmt))
            .repeated()
            .map(Block)
            .delimited_by(just(Token::LCurly), just(Token::RCurly))
    }
}
