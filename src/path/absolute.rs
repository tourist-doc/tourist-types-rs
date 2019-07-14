use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq)]
pub struct AbsolutePathBuf(PathBuf);

impl AbsolutePathBuf {
    pub fn new(p: PathBuf) -> Option<Self> {
        if p.is_absolute() {
            Some(AbsolutePathBuf(p))
        } else {
            None
        }
    }

    pub fn new_unchecked(p: PathBuf) -> Self {
        debug_assert!(p.is_absolute());
        AbsolutePathBuf(p)
    }

    pub fn as_path_buf(&self) -> &PathBuf {
        &self.0
    }

    pub fn as_absolute_path(&self) -> AbsolutePath<'_> {
        AbsolutePath(&self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct AbsolutePath<'a>(&'a Path);

impl<'a> AbsolutePath<'a> {
    pub fn new(p: &'a Path) -> Option<Self> {
        if p.is_absolute() {
            Some(AbsolutePath(p))
        } else {
            None
        }
    }

    pub fn new_unchecked(p: &'a Path) -> Self {
        debug_assert!(p.is_absolute());
        AbsolutePath(p)
    }

    pub fn as_path(&self) -> &Path {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::AbsolutePathBuf;
    use dirs;
    use std::path::Path;

    #[test]
    fn create_abs_path() {
        let abs = dirs::home_dir().unwrap().join("some").join("path");
        let not_abs = Path::new("some").join("path");
        assert_eq!(
            &abs.clone(),
            AbsolutePathBuf::new(abs).unwrap().as_path_buf()
        );
        assert!(AbsolutePathBuf::new(not_abs).is_none());
    }
}
