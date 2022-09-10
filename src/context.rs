use clap::Parser;

use crate::args::CliArgs;
use crate::cache::FileCache;

pub struct Context {
    pub args: CliArgs,
    pub cache: FileCache,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            args: CliArgs::parse(),
            cache: FileCache::default(),
        }
    }
}
