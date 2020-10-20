use super::Direction;
use crate::migration::{Change, Migration};
use std::slice::Iter;

pub struct QueryIter<'a> {
    migration: &'a Migration,
    inner: Option<Iter<'a, String>>,
    idx: usize,
    direction: Direction,
}

impl<'a> QueryIter<'a> {
    pub fn new(migration: &'a Migration, direction: Direction) -> Self {
        Self {
            migration,
            idx: 0,
            inner: None,
            direction,
        }
    }

    fn next(&mut self) -> Option<&'a String> {
        match self.migration.changes.get(self.idx) {
            Some(change_set) => {
                self.idx += 1;
                let maybe_change = match self.direction {
                    Direction::Up => Some(&change_set.up),
                    Direction::Down => change_set.down.as_ref(),
                };

                let change = match maybe_change {
                    Some(change) => change,
                    None => return self.next(),
                };

                match change {
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
