use crate::lexer::Token;
use ariadne::{Color, Fmt, Label};
use chumsky::error::SimpleReason;
use chumsky::prelude::*;
use std::ops::Range;
use std::path::Path;

pub enum Error {
    Lex(Vec<Simple<char>>),
    Parse(Vec<Simple<Token>>),
    Syntax(Vec<Simple<Token>>),
}

pub fn convert_non_lex_errors(
    ranges: &[Range<usize>],
    err: Vec<Simple<Token>>,
) -> Vec<Simple<Token>> {
    err.into_iter()
        .map(|x| {
            let token_span = x.span();
            let range = ranges[token_span.start].start..ranges[token_span.end - 1].end;
            Simple::custom(range, display_error_reason(x.reason()))
        })
        .collect()
}

fn display_error_reason(res: &SimpleReason<Token, Range<usize>>) -> String {
    match res {
        SimpleReason::Custom(msg) => format!("{msg}"),
        SimpleReason::Unexpected => format!("Unexpected Token"),
        SimpleReason::Unclosed { delimiter, .. } => {
            format!("Unclosed delimiter: {delimiter:?}")
        }
    }
}

pub fn custom_error_label<'a>(
    path: &'a Path,
    span: Range<usize>,
    msg: &String,
) -> Label<(&'a Path, Range<usize>)> {
    Label::new((path, span)).with_message(format!("{}", msg.fg(crate::colors::RED)))
}

pub fn unclosed_delim_label<'a>(
    path: &'a Path,
    span: Range<usize>,
    delimiter: &String,
) -> Label<(&'a std::path::Path, Range<usize>)> {
    Label::new((path, span)).with_message(format!(
        "Unclosed delimiter {}",
        delimiter.fg(crate::colors::YELLOW)
    ))
}

pub fn unclosed_delim_msg(delimiter: &String) -> String {
    format!("Unclosed delimiter {}", delimiter.fg(crate::colors::YELLOW))
}

pub fn unclosed_delim_label_2<'a>(
    path: &'a Path,
    e: &Simple<String>,
) -> Label<(&'a Path, Range<usize>)> {
    Label::new((path, e.span())).with_message(format!(
        "Must be closed before this {}",
        e.found()
            .unwrap_or(&"end of file".to_string())
            .fg(crate::colors::RED)
    ))
}

pub fn unexpected_label<'a>(path: &'a Path, e: &Simple<String>) -> Label<(&'a Path, Range<usize>)> {
    Label::new((path, e.span()))
        .with_color(Color::Cyan)
        .with_message(format!(
            "Unexpected token {}",
            e.found()
                .unwrap_or(&"end of file".to_string())
                .fg(Color::Red)
        ))
}

pub fn unexpected_msg(e: &Simple<String>) -> String {
    format!(
        "{}, expected {}",
        if e.found().is_some() {
            "Unexpected token in input"
        } else {
            "Unexpected end of input"
        },
        if e.expected().len() == 0 {
            "something else".to_string()
        } else {
            e.expected()
                .map(|expected| match expected {
                    Some(expected) => expected.to_string(),
                    None => "end of input".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        }
    )
}
