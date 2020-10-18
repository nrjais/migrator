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
        if let Some(change_set) = self.migration.changes.get(self.idx) {
            self.idx += 1;
            match change_set.up {
                Change::Query { ref query } => Some(query),
                Change::Queries { ref queries } => {
                    self.inner = Some(queries.iter());
                    Iterator::next(self)
                }
                Change::SqlFile { .. } => None,
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for QueryIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.inner {
            let value = inner.next();
            if value.is_none() {
                self.inner = None;
                Self::next(self)
            } else {
                value
            }
        } else {
            Self::next(self)
        }
    }
}
