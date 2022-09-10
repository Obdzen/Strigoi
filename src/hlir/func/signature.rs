use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::hlir::{FnArg, Macro, Type, Vis};
use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct FnSig {
    pub macros: Vec<Macro>,
    pub vis: Option<Vis>,
    pub name: String,
    pub args: Vec<FnArg>,
    pub ret: Option<Type>,
}

impl FnSig {
    pub(crate) fn parser() -> impl Parser<Token, FnSig, Error = Simple<Token>> {
        let ident = select! { Token::Ident(data) => data.clone() };
        Macro::parser()
            .repeated()
            .then(Vis::parser().or_not())
            .then_ignore(just(Token::Fn))
            .then(ident.clone())
            .then(
                FnArg::parser()
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::LParan), just(Token::RParan)),
            )
            .then(
                just(&[Token::Sub, Token::GreaterThan])
                    .ignore_then(Type::parser())
                    .or_not(),
            )
            .map(|((((macros, vis), name), args), ret)| FnSig {
                macros,
                vis,
                name,
                args,
                ret,
            })
    }
}
