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

use crate::types::Id;

#[derive(Debug, Hash, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum EntityId {
    Champion(Id),
    Spell(Id),
    Equip(Id),
    Relic(Id),
    AbilityGroup(Id),
    Effect(Id),
}

pub struct Tags {
    inner: Vec<EntityId>,
}

impl fmt::Debug for Tags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Default for Tags {
    fn default() -> Self {
        Tags {
            inner: Vec::new(),
        }
    }
}

impl Tags {
    pub fn tag(&mut self, taggable: impl Into<EntityId>) -> &mut Self {
        let tid = taggable.into();

        if let Err(index) = self.inner.binary_search(&tid) {
            self.inner.insert(index, tid);
        }

        self
    }
}
