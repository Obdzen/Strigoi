use crate::lexer::Token;

use super::Expr;
use chumsky::prelude::*;
use dbg_pls::DebugPls;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct Stmt {
    pub terminated: bool,
    pub statement: Statement,
}

impl Stmt {
    pub(crate) fn parser() -> impl Parser<Token, Stmt, Error = Simple<Token>> {
        Statement::parser()
            .then(just(Token::SemiColon).or_not().map(|s| s.is_some()))
            .map(|(statement, terminated)| Stmt {
                statement,
                terminated,
            })
    }
}

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub enum Statement {
    Expr(Expr),
    Let(String, Expr),
    Return(Option<Expr>),
    Call(String, Vec<Expr>),
}

impl Statement {
    pub(crate) fn parser() -> impl Parser<Token, Statement, Error = Simple<Token>> {
        let ident = select! { Token::Ident(data) => data.clone() };
        choice((
            just(Token::Let)
                .ignore_then(ident)
                .then_ignore(just(Token::Equal))
                .then(Expr::parser())
                .map(|(a, b)| Statement::Let(a, b)),
            ident
                .then(
                    super::Expr::parser()
                        .separated_by(just(Token::Comma))
                        .delimited_by(just(Token::LParan), just(Token::RParan)),
                )
                .map(|(ident, args)| Statement::Call(ident, args)),
            just(Token::Return).ignore_then(Expr::parser().or_not().map(Statement::Return)),
            Expr::parser().map(Statement::Expr),
        ))
    }
}
