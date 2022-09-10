use chumsky::prelude::*;
use walkdir::WalkDir;

use crate::lexer::Token;

#[test]
fn samples() {
    for entry in WalkDir::new("tests/data/samples") {
        let entry = entry.unwrap();
        if let Some(ext) = entry.path().extension() {
            if ext.to_str().unwrap() == "oi" {
                use std::io::Read;
                let mut file = std::fs::File::open(entry.path()).unwrap();
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                let lex_response = Token::lexer().parse(data.as_str()).unwrap();
                let tokens = lex_response
                    .into_iter()
                    .map(|(a, _)| a)
                    .collect::<Vec<Token>>();
                let ast = crate::hlir::Ast::parse(&tokens);
                assert!(ast.is_ok());
            }
        }
    }
}
