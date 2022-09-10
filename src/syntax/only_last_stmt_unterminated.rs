use crate::{frontend::SyntaxPass, hlir::Item, lexer::Token};
use chumsky::prelude::*;

pub struct OnlyLastStatementUnterminated;

impl SyntaxPass for OnlyLastStatementUnterminated {
    fn pass(&mut self, ast: &mut crate::hlir::Ast) -> Result<(), Vec<Simple<Token>>> {
        for item in ast.0.iter() {
            match item {
                Item::Fn(def) => {
                    for stmt in def
                        .block
                        .0
                        .iter()
                        .take(def.block.0.len() - (def.block.0.len() * 1))
                    {
                        if !stmt.1.terminated {
                            return Err(vec![Simple::custom(
                                stmt.0.clone(),
                                "Only The final statement in a block may be unterminated",
                            )]);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
