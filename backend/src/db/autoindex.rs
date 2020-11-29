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

use std::fmt;
use std::ops::{Index, IndexMut};
use std::hash::Hash;
use std::borrow::Borrow;
use rustc_hash::FxHashMap;

use crate::types::Id;
use crate::db::TableItem;

pub struct AutoIndexTable<T: AutoIndexItem> {
    ids: FxHashMap<T::Key, Id>,
    table: Vec<T>,
}

impl<T> fmt::Debug for AutoIndexTable<T>
where
    T: fmt::Debug + AutoIndexItem,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.table.iter().map(|entry| (entry.id(), entry)))
            .finish()
    }
}

impl<T: AutoIndexItem> Default for AutoIndexTable<T> {
    fn default() -> Self {
        AutoIndexTable {
            ids: FxHashMap::default(),
            table: Vec::new(),
        }
    }
}

impl<T: AutoIndexItem> AutoIndexTable<T> {
    pub fn entry(&mut self, key: &str) -> &mut T {
        if let Some(id) = self.ids.get(key) {
            return &mut self.table[*id as usize];
        }

        let id = self.table.len() as Id;
        let key = T::make_key(key);

        self.table.push(T::create(id, key.clone()));
        self.ids.insert(key, id);

        &mut self.table[id as usize]
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        self.table.get(id as usize)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.table.get_mut(id as usize)
    }

    pub fn get_by_key(&self, key: &str) -> Option<&T> {
        let id = self.ids.get(key)?;

        self.table.get(*id as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.table.iter()
    }
}

impl<T: AutoIndexItem> Index<Id> for AutoIndexTable<T> {
    type Output = T;

    fn index(&self, id: Id) -> &Self::Output {
        self.table.index(id as usize)
    }
}

impl<T: AutoIndexItem> IndexMut<Id> for AutoIndexTable<T> {
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        self.table.index_mut(id as usize)
    }
}

pub trait AutoIndexItem: TableItem {
    type Key: Clone + Eq + Hash + Borrow<str>;

    fn create(id: Id, key: Self::Key) -> Self;

    fn make_key(key: &str) -> Self::Key;
}
