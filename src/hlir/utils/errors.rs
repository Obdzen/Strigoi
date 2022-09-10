use ariadne::{Report, ReportKind};
use chumsky::prelude::*;
use std::path::Path;

use crate::cache::FileCache;
use crate::errors::*;

pub fn report_parse_errors(path: &Path, mut file_cache: &mut FileCache, errs: Vec<Simple<String>>) {
    for e in errs.into_iter() {
        let report = Report::build(ReportKind::Error, path, e.span().start);

        let report = match e.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                .with_message(unclosed_delim_msg(delimiter))
                .with_label(
                    unclosed_delim_label(path, span.clone(), delimiter)
                        .with_color(crate::colors::ORANGE),
                )
                .with_label(unclosed_delim_label_2(path, &e)),
            chumsky::error::SimpleReason::Unexpected => report
                .with_message(unexpected_msg(&e))
                .with_label(unexpected_label(path, &e).with_color(crate::colors::ORANGE)),
            chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                crate::errors::custom_error_label(path, e.span(), msg)
                    .with_color(crate::colors::ORANGE),
            ),
        };

        report.finish().print(&mut file_cache).unwrap();
    }
}
