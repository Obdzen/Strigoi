use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    #[clap(value_parser, value_name = "IN")]
    pub in_file: PathBuf,
    #[clap(short, long, value_parser, value_name = "OUT")]
    pub out_file: Option<PathBuf>,
    #[clap(long = "d-tokens")]
    pub debug_tokens: bool,
    #[clap(long = "d-ast")]
    pub debug_ast: bool,
    #[clap(long = "d-llvm-ir")]
    pub debug_llvm_ir: bool,
    #[clap(long = "d-asm")]
    pub debug_asm: bool,
}
