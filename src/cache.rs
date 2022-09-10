use ariadne::{Cache, Color, Fmt, Source};
use std::collections::{hash_map::Entry, HashMap};
use std::path::{Path, PathBuf};
use std::{fmt, fs};

pub struct FileCache {
    files: HashMap<PathBuf, Source>,
}

impl Default for FileCache {
    fn default() -> Self {
        Self {
            files: HashMap::default(),
        }
    }
}

impl<'a> Cache<&'a Path> for FileCache {
    fn fetch(&mut self, path: &&'a Path) -> Result<&Source, Box<dyn fmt::Debug + '_>> {
        Ok(match self.files.entry(path.to_path_buf()) {
            // TODO: Don't allocate here
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Source::from(
                &fs::read_to_string(path).map_err(|e| Box::new(e) as _)?,
            )),
        })
    }
    fn display<'b>(&self, path: &&'b Path) -> Option<Box<dyn fmt::Display + 'b>> {
        Some(Box::new(path.display().fg(Color::Magenta)))
    }
}
