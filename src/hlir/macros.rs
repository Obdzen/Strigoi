use super::Value;
use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone, Copy, Hash, Eq)]
pub enum MacroInvocation {
    Compiler,
    Syntax,
}

impl MacroInvocation {
    pub(crate) fn parser() -> impl Parser<Token, MacroInvocation, Error = Simple<Token>> {
        choice((
            just(Token::At).to(MacroInvocation::Syntax),
            just(Token::Pound).to(MacroInvocation::Compiler),
        ))
    }
}

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct Macro {
    pub inv: MacroInvocation,
    pub ident: String,
    pub args: Option<Vec<Value>>,
}

impl Macro {
    pub fn parser() -> impl Parser<Token, Macro, Error = Simple<Token>> {
        let ident = select! { Token::Ident(data) => data.clone() };
        MacroInvocation::parser()
            .then(ident)
            .then(
                Value::parser()
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::LParan), just(Token::RParan))
                    .or_not(),
            )
            .map(|((inv, ident), args)| Macro { inv, ident, args })
    }
}
