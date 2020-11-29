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

use arrayvec::ArrayString;
use serde::Serialize;
use serde_repr::Serialize_repr;

use crate::db::{AutoIndexItem, TableItem, Searchable, SearchId};
use crate::types::{Id, Tags};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub id: Id,
    pub key: ArrayString<[u8; 15]>,
    pub name: Box<str>,
    pub desc: Box<str>,
    pub kind: EffectKind,
    #[serde(skip)]
    pub tags: Tags,
}

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum EffectKind {
    None,
    Damage,
    Condition,
}

impl TableItem for Effect {
    fn id(&self) -> Id {
        self.id
    }
}

impl AutoIndexItem for Effect {
    type Key = ArrayString<[u8; 15]>;

    fn create(id: Id, key: Self::Key) -> Self {
        Effect {
            id,
            key,
            name: "".into(),
            desc: "".into(),
            kind: EffectKind::None,
            tags: Tags::default(),
        }
    }

    fn make_key(key: &str) -> Self::Key {
        let mut key = ArrayString::from(key).expect("Key in Map is too large");

        key.make_ascii_lowercase();
        key
    }
}

impl Searchable for Effect {
    fn search_id(&self) -> SearchId {
        let key = &self.key;

        match self.kind {
            EffectKind::Condition => SearchId::Condition(key),
            EffectKind::Damage => SearchId::Damage(key),
            EffectKind::None => SearchId::Effect(key),
        }
    }
}
