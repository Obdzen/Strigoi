use chumsky::prelude::*;
use ariadne::Cache;
use std::ops::Range;

use crate::hlir::Ast;
use crate::lexer::Token;
use crate::Context;

pub trait SyntaxPass {
    fn pass(&mut self, ast: &mut Ast) -> Result<(), Vec<Simple<Token>>>;
}

pub struct Frontend<'a> {
    pub ctx: &'a mut Context,
    pub passes: Vec<Box<dyn SyntaxPass>>,
}

impl<'a> Frontend<'a> {
    pub fn new(ctx: &'a mut Context) -> Frontend {
        Frontend {
            ctx,
            passes: vec![],
        }
    }

    pub fn add_pass<P: SyntaxPass + 'static>(&mut self, pass: P) -> &mut Self {
        self.passes.push(Box::new(pass));
        self
    }

    pub fn lex(&mut self) -> Result<(String, Vec<(Token, Range<usize>)>), Vec<Simple<char>>> {
        let mut source = String::new();
        for line in self.ctx.cache.fetch(&self.ctx.args.in_file.as_path()).map_err(|x|vec![Simple::custom(0..0, format!("{:?}", x))])?.lines() {
            line.chars().for_each(|c|source.push(c));
            source.push('\n');
        }
        let tokens = Token::lexer().parse(source.as_str())?;
        Ok((source, tokens))
    }

    pub fn parse(&mut self, tokens: &[Token]) -> Result<Ast, Vec<Simple<Token>>> {
        Ast::parse(tokens)
    }

    pub fn run_syntax_passes(&mut self, ast: &mut Ast) -> Result<(), Vec<Simple<Token>>> {
        for pass in self.passes.iter_mut() {
            pass.pass(ast)?;
        }
        Ok(())
    }
}
