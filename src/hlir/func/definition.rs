use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::hlir::{Block, FnSig};
use crate::lexer::Token;

#[derive(Debug, DebugPls, PartialEq, Clone)]
pub struct FnDef {
    pub sig: FnSig,
    pub block: Block,
}

impl FnDef {
    pub(crate) fn parser() -> impl Parser<Token, FnDef, Error = Simple<Token>> {
        FnSig::parser()
            .then(Block::parser())
            .map(|(sig, block)| FnDef { sig, block })
    }
}
