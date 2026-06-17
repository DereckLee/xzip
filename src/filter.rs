use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::error::RzipError;

#[derive(Debug)]
pub struct PathFilter {
    include: Option<GlobSet>,
    exclude: Option<GlobSet>,
}

impl PathFilter {
    pub fn new(include: &[String], exclude: &[String]) -> Result<Self, RzipError> {
        Ok(Self {
            include: compile_globs(include)?,
            exclude: compile_globs(exclude)?,
        })
    }

    pub fn allows(&self, path: &str) -> bool {
        let include_ok = self
            .include
            .as_ref()
            .map(|set| set.is_match(path))
            .unwrap_or(true);
        let excluded = self
            .exclude
            .as_ref()
            .map(|set| set.is_match(path))
            .unwrap_or(false);
        include_ok && !excluded
    }
}

fn compile_globs(patterns: &[String]) -> Result<Option<GlobSet>, RzipError> {
    if patterns.is_empty() {
        return Ok(None);
    }

    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob =
            Glob::new(pattern).map_err(|_| RzipError::InvalidGlobPattern(pattern.clone()))?;
        builder.add(glob);
    }
    let set = builder
        .build()
        .map_err(|_| RzipError::InvalidGlobPattern("failed to build glob set".to_string()))?;
    Ok(Some(set))
}
