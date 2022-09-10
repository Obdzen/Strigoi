use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Vis {
    Public,
}

impl Vis {
    pub(crate) fn parser() -> impl Parser<Token, Vis, Error = Simple<Token>> {
        just(Token::Pub).to(Vis::Public)
    }
}
