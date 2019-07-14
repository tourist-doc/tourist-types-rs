use std::path::PathBuf;

pub type Component = String;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RelativePathBuf(Vec<Component>);

impl RelativePathBuf {
    pub fn join(mut self, c: Component) -> Self {
        self.0.push(c);
        self
    }

    pub fn from_components<I: Iterator<Item = Component>>(i: I) -> Self {
        RelativePathBuf(i.collect())
    }

    pub fn as_path_buf(&self) -> PathBuf {
        let mut p = PathBuf::new();
        self.0.iter().for_each(|c| p.push(c));
        p
    }

    pub fn as_git_path(&self) -> String {
        self.0.to_vec().join("/")
    }
}

impl From<&str> for RelativePathBuf {
    fn from(s: &str) -> Self {
        RelativePathBuf::from_components(s.split('/').filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(x.to_owned())
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::RelativePathBuf;

    #[test]
    fn join_works() {
        let mut path = RelativePathBuf(vec![]).join("some".to_owned());
        assert_eq!(path.0.len(), 1);
        assert_eq!(path.0[0], "some");

        path = path.join("dir".to_owned());
        assert_eq!(path.0.len(), 2);
        assert_eq!(path.0[0], "some");
        assert_eq!(path.0[1], "dir");
    }

    #[test]
    fn from_components_works() {
        let path =
            RelativePathBuf::from_components(vec!["some".to_owned(), "dir".to_owned()].into_iter());
        assert_eq!(path.0.len(), 2);
        assert_eq!(path.0[0], "some");
        assert_eq!(path.0[1], "dir");
    }

    #[test]
    fn from_str_works() {
        {
            let path: RelativePathBuf = "some/dir".into();
            assert_eq!(path.0.len(), 2);
            assert_eq!(path.0[0], "some");
            assert_eq!(path.0[1], "dir");
        }

        {
            let path: RelativePathBuf = "some".into();
            assert_eq!(path.0.len(), 1);
            assert_eq!(path.0[0], "some");
        }

        {
            let path: RelativePathBuf = "".into();
            assert_eq!(path.0.len(), 0);
        }

        {
            let path: RelativePathBuf = "/some//dir/".into();
            assert_eq!(path.0.len(), 2);
            assert_eq!(path.0[0], "some");
            assert_eq!(path.0[1], "dir");
        }
    }
}
