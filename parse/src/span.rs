use anyhow::Result;
use chumsky::span::{SimpleSpan, Span as _};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::RwLock,
};

use ariadne::{Cache, Source};

use crate::arena::{Arena, Id};

pub struct FileCache {
    files: Arena<Source, 8>,
    paths: RwLock<HashMap<PathBuf, Id<Source>>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self {
            files: Arena::new(),
            paths: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, id: Id<Source>) -> &Source {
        self.files.get(id)
    }

    pub fn resolve(&self, path: impl AsRef<Path>) -> Result<Id<Source>> {
        let path = path.as_ref();
        let mut paths = self.paths.write().expect("to acquire write access");
        if let Some(id) = paths.get(path) {
            Ok(*id)
        } else {
            let str = std::fs::read_to_string(&path)?;
            let source = Source::from(str);
            let id = self.files.insert(source);
            paths.insert(path.to_path_buf(), id);
            Ok(id)
        }
    }

    pub fn resolve_and_get(&self, path: impl AsRef<Path>) -> Result<&Source> {
        let id = self.resolve(path)?;
        Ok(self.get(id))
    }
}

impl Cache<Path> for FileCache {
    fn fetch(&mut self, path: &Path) -> Result<&Source, Box<dyn std::fmt::Debug + '_>> {
        self.resolve_and_get(path)
            .map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug + '_>)
    }

    fn display<'b>(&self, path: &'b Path) -> Option<Box<dyn std::fmt::Display + 'b>> {
        self.resolve_and_get(path).ok()?.display(&())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
    source: Id<Source>,
}

impl Span {
    pub fn new(span: SimpleSpan, source: Id<Source>) -> Self {
        Self {
            start: span.start(),
            end: span.end(),
            source,
        }
    }
}

impl ariadne::Span for Span {
    type SourceId = Id<Source>;

    fn source(&self) -> &Self::SourceId {
        &self.source
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

impl chumsky::span::Span for Span {
    type Context = Id<Source>;

    type Offset = usize;

    fn new(source: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self {
            source,
            start: range.start,
            end: range.end,
        }
    }

    fn context(&self) -> Self::Context {
        self.source
    }

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

pub type Spanned<T> = (T, Span);
