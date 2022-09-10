use chumsky::prelude::*;
use dbg_pls::DebugPls;
use std::{num::ParseIntError, str::FromStr};

use crate::lexer::Token;

pub enum IntegerTypeParseError {
    Int(ParseIntError),
    InvalidPrefix,
}

impl From<ParseIntError> for IntegerTypeParseError {
    fn from(f: ParseIntError) -> IntegerTypeParseError {
        IntegerTypeParseError::Int(f)
    }
}

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct IntegerType {
    pub signed: bool,
    pub bits: u8,
}

impl FromStr for IntegerType {
    type Err = IntegerTypeParseError;
    fn from_str(s: &str) -> Result<IntegerType, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'i' => Ok(IntegerType {
                signed: true,
                bits: FromStr::from_str(&s[1..])?,
            }),
            'u' => Ok(IntegerType {
                signed: false,
                bits: FromStr::from_str(&s[1..])?,
            }),
            _ => Err(IntegerTypeParseError::InvalidPrefix),
        }
    }
}

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub enum Type {
    Bool,
    Int(IntegerType),
}

impl FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(ty) = IntegerType::from_str(s) {
            Ok(Type::Int(ty))
        } else {
            if s == "bool" {
                Ok(Type::Bool)
            } else {
                Err(())
            }
        }
    }
}

impl Type {
    pub(crate) fn parser() -> impl Parser<Token, Type, Error = Simple<Token>> {
        select! { Token::Ident(data) => data.clone() }
            .from_str()
            .unwrapped()
    }
}
