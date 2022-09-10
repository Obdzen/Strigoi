use super::func::{FnDef, FnSig};
use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub enum Item {
    Fn(FnDef),
    ExtFn(FnSig),
}

impl Item {
    pub(crate) fn parser() -> impl Parser<Token, Item, Error = Simple<Token>> {
        choice((
            FnDef::parser().map(Item::Fn),
            just(Token::Extern)
                .ignore_then(FnSig::parser())
                .map(Item::ExtFn),
        ))
    }
}
