use crate::hlir::{Macro, Type};
use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct FnArg {
    pub macros: Vec<Macro>,
    pub name: String,
    pub ty: Option<Type>,
}

impl FnArg {
    pub(crate) fn parser() -> impl Parser<Token, FnArg, Error = Simple<Token>> {
        let ident = select! { Token::Ident(data) => data.clone() };
        Macro::parser()
            .repeated()
            .then(ident)
            .then(just(Token::Colon).ignore_then(Type::parser()).or_not())
            .map(|((macros, name), ty)| FnArg { macros, name, ty })
    }
}
