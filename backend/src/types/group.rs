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

use std::marker::PhantomData;
use serde::Serialize;

use crate::types::{Id, Ability, EntityId, Tags};
use crate::db::{AutoIndexItem, TableItem};

#[derive(Debug)]
pub struct Group<T> {
    pub id: Id,
    pub name: Box<str>,
    pub ids: Vec<Id>,
    phantom: PhantomData<T>,
}

#[derive(Debug, Serialize)]
pub struct Shim<'a> {
    pub id: Id,
    pub name: &'a str,
}

impl<T> Group<T> {
    pub fn shim(&self) -> Shim {
        Shim {
            id: self.id,
            name: &self.name,
        }
    }

    pub fn add(&mut self, item: &T) -> &mut Self
    where
        T: TableItem
    {
        self.ids.push(item.id());
        self
    }
}

impl<T> AutoIndexItem for Group<T> {
    type Key = Box<str>;

    fn create(id: Id, name: Self::Key) -> Self {
        Group {
            id,
            name,
            ids: Vec::new(),
            phantom: PhantomData,
        }
    }

    fn make_key(key: &str) -> Self::Key {
        key.into()
    }
}

impl<T> TableItem for Group<T> {
    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Debug, Serialize)]
pub struct AbilityGroup {
    pub id: Id,
    pub name: Box<str>,
    pub ranks: Vec<Id>,
    #[serde(skip)]
    pub tags: Tags,
}

impl AbilityGroup {
    pub fn shim(&self) -> Shim {
        Shim {
            id: self.id,
            name: &self.name,
        }
    }

    pub fn rank(&mut self, rank: &Ability) -> &mut Self {
        let id = rank.id();

        if let Err(index) = self.ranks.binary_search(&id) {
            self.ranks.insert(index, id);
        }

        self
    }

    pub fn tag(&mut self, taggable: impl Into<EntityId>) -> &mut Self {
        self.tags.tag(taggable);
        self
    }
}

impl AutoIndexItem for AbilityGroup {
    type Key = Box<str>;

    fn create(id: Id, name: Self::Key) -> Self {
        AbilityGroup {
            id,
            name,
            ranks: Vec::new(),
            tags: Tags::default(),
        }
    }

    fn make_key(key: &str) -> Self::Key {
        key.into()
    }
}

impl TableItem for AbilityGroup {
    fn id(&self) -> Id {
        self.id
    }
}
