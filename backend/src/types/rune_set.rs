// PoxBase
// Copyright (C) 2020  Maciej Hirsz
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::db::{TableItem, AutoIndexItem};
use crate::types::{Id, EntityId, Shim, Tags};

#[derive(Debug)]
pub struct RuneSet<K = Box<str>> {
    pub id: Id,
    pub name: K,
    pub tags: Tags,
}

impl RuneSet<Box<str>> {
    pub fn shim(&self) -> Shim {
        Shim {
            id: self.id,
            name: &self.name,
        }
    }
}

impl<K> RuneSet<K> {
    pub fn tag(&mut self, taggable: impl Into<EntityId>) -> &mut Self {
        self.tags.tag(taggable);
        self
    }
}

impl<K> From<K> for RuneSet<K>
where
    K: TableItem + Copy,
{
    fn from(key: K) -> Self {
        RuneSet {
            id: key.id(),
            name: key,
            tags: Tags::default(),
        }
    }
}

impl AutoIndexItem for RuneSet<Box<str>> {
    type Key = Box<str>;

    fn create(id: Id, name: Self::Key) -> Self {
        RuneSet {
            id,
            name,
            tags: Tags::default(),
        }
    }

    fn make_key(key: &str) -> Self::Key {
        key.into()
    }
}

impl<K> TableItem for RuneSet<K> {
    fn id(&self) -> Id {
        self.id
    }
}
