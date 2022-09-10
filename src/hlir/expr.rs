use super::{Op, Spanned, Value};
use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub enum Expr {
    Value(Spanned<Value>),
    Op(Box<Expr>, Op, Box<Expr>),
}

impl Expr {
    pub(crate) fn parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
        recursive(|tree| {
            choice((
                Value::parser()
                    .then(Op::parser())
                    .then(tree.clone())
                    .map_with_span(|((value, op), expr), spn| {
                        Expr::Op(Box::new(Expr::Value((spn, value))), op, Box::new(expr))
                    }),
                tree.clone()
                    .map(Box::new)
                    .then(Op::parser())
                    .then(tree.map(Box::new))
                    .delimited_by(just(Token::LParan), just(Token::RParan))
                    .map(|((l, op), r)| Expr::Op(l, op, r)),
                Value::parser().map_with_span(|v, spn| Expr::Value((spn, v))),
            ))
        })
    }
}
