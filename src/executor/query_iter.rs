use crate::migration::{Change, Migration};
use std::slice::Iter;

pub struct QueryIter<'a> {
    migration: &'a Migration,
    inner: Option<Iter<'a, String>>,
    idx: usize,
}

impl<'a> QueryIter<'a> {
    pub fn new(migration: &'a Migration) -> Self {
        Self {
            migration,
            idx: 0,
            inner: None,
        }
    }

    fn next(&mut self) -> Option<&'a String> {
        match self.migration.changes.get(self.idx) {
            Some(change_set) => {
                self.idx += 1;
                match change_set.up {
                    Change::Query { ref query } => Some(query),
                    Change::Queries { ref queries } => {
                        self.inner = Some(queries.iter());
                        Iterator::next(self)
                    }
                    Change::SqlFile { .. } => None,
                }
            }
            None => None,
        }
    }
}

impl<'a> Iterator for QueryIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner {
            Some(ref mut inner) => inner.next().map(Option::Some).unwrap_or_else(|| {
                self.inner = None;
                Self::next(self)
            }),
            None => Self::next(self),
        }
    }
}
