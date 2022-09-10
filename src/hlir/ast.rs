use super::Item;
use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct Ast(pub Vec<Item>);

impl Ast {
    pub(crate) fn parser() -> impl Parser<Token, Ast, Error = Simple<Token>> {
        Item::parser()
            .repeated()
            .at_least(1)
            .then_ignore(end())
            .map(Ast)
    }

    pub fn parse(input: &[Token]) -> Result<Ast, Vec<Simple<Token>>> {
        Ok(Ast::parser().parse(input)?)
    }
}
