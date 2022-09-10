mod args;
mod cache;
mod colors;
mod context;
mod errors;
mod frontend;
mod hlir;
mod lexer;
mod syntax;

use context::Context;
use frontend::Frontend;
use lexer::Token;
use std::ops::Range;

fn main() {
    let mut context = Context::default();
    if let Err(err) = inner(&mut context) {
        match err {
            errors::Error::Lex(errs) => {
                let errs = errs.into_iter().map(|e| e.map(|c| c.to_string())).collect();
                crate::lexer::report_lex_errors(&context.args.in_file, &mut context.cache, errs);
            }
            errors::Error::Parse(errs) => {
                let errs = errs.into_iter().map(|e| e.map(|c| c.to_string())).collect();
                crate::hlir::report_parse_errors(&context.args.in_file, &mut context.cache, errs);
            }
            errors::Error::Syntax(errs) => {
                let errs = errs.into_iter().map(|e| e.map(|c| c.to_string())).collect();
                crate::syntax::report_syntax_errors(
                    &context.args.in_file,
                    &mut context.cache,
                    errs,
                );
            }
        }
    }
}

fn inner(context: &mut Context) -> Result<(), errors::Error> {
    let mut frontend = Frontend::new(context);
    let (_source, _tokens) = frontend.lex().map_err(errors::Error::Lex)?;
    let (tokens, ranges): (Vec<Token>, Vec<Range<usize>>) = _tokens.into_iter().unzip();

    if frontend.ctx.args.debug_tokens {
        println!("{}", dbg_pls::color(&tokens));
        std::process::exit(0);
    }

    let mut ast = frontend
        .parse(&tokens)
        .map_err(|err| errors::convert_non_lex_errors(&ranges, err))
        .map_err(errors::Error::Parse)?;

    syntax::build_syntax_passes(&mut frontend);
    frontend
        .run_syntax_passes(&mut ast)
        .map_err(|err| errors::convert_non_lex_errors(&ranges, err))
        .map_err(errors::Error::Syntax)?;

    if context.args.debug_ast {
        println!("{:?}", dbg_pls::color(&ast));
        std::process::exit(0);
    }

    Ok(())
}
