use chumsky::prelude::*;

use walkdir::WalkDir;

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
                assert!(super::Token::lexer().parse(data.as_str()).is_ok());
            }
        }
    }
}
