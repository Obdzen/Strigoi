mod errors;
mod only_last_stmt_unterminated;

use crate::frontend::Frontend;

pub use self::errors::report_syntax_errors;
use self::only_last_stmt_unterminated::OnlyLastStatementUnterminated;

pub fn build_syntax_passes(frontend: &mut Frontend) {
    frontend.add_pass(OnlyLastStatementUnterminated);
}
