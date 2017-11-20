use failure::{Context, Fail};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoMatch {
    pos: usize,
    parser: &'static str,
    parents: Vec<NoMatch>,
}

impl Fail for NoMatch {
    fn cause(&self) -> Option<&Fail> {
        self.parents.iter().next()
    }
}
