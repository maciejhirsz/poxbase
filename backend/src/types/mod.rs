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

use serde::{Serialize, Deserialize};
use arrayvec::{ArrayVec, ArrayString};

use crate::db::TableItem;

mod enums;
mod effect;
mod ability;
mod champ;
mod group;
mod rune_set;
mod tags;

pub use enums::{Size, Rarity, Faction};
pub use effect::{Effect, EffectKind};
pub use ability::{Ability, AbilityCore};
pub use champ::{Champion, ChampionCore};
pub use group::{AbilityGroup, Group, Shim};
pub use rune_set::RuneSet;
pub use tags::{Tags, EntityId};

pub type Id = u32;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuneRaw {
    pub id: Id,
    pub name: Box<str>,
    pub description: Box<str>,
    pub rarity: Rarity,
    pub nora_cost: u16,
    pub for_sale: bool,
    pub tradeable: bool,
    pub allow_ranked: bool,
    pub hash: ArrayString<[u8; 40]>,
    pub deck_limit: u8,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rune<C> {
    #[serde(flatten)]
    pub core: C,
    pub factions: ArrayVec<[Faction; 2]>,
    pub expansion: Id,
    pub artist: Id,
}

impl<C> From<C> for Rune<C> {
    fn from(core: C) -> Self {
        Rune {
            core,
            factions: ArrayVec::new(),
            expansion: !0,
            artist: !0,
        }
    }
}

impl<C> TableItem for Rune<C>
where
    C: TableItem,
{
    fn id(&self) -> Id {
        self.core.id()
    }
}

impl From<&Rune<Spell>> for EntityId {
    fn from(taggable: &Rune<Spell>) -> EntityId {
        EntityId::Spell(taggable.id())
    }
}

impl From<&Rune<Equip>> for EntityId {
    fn from(taggable: &Rune<Equip>) -> EntityId {
        EntityId::Equip(taggable.id())
    }
}

impl From<&Rune<Relic>> for EntityId {
    fn from(taggable: &Rune<Relic>) -> EntityId {
        EntityId::Relic(taggable.id())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    #[serde(flatten)]
    pub raw: RuneRaw,
    pub flavor_text: Box<str>,
    pub cooldown: u8,
}

impl TableItem for Spell {
    fn id(&self) -> Id {
        self.raw.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Equip {
    #[serde(flatten)]
    pub raw: RuneRaw,
    pub flavor_text: Box<str>,
}

impl TableItem for Equip {
    fn id(&self) -> Id {
        self.raw.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    #[serde(flatten)]
    pub raw: RuneRaw,
    pub flavor_text: Box<str>,
    pub defense: u16,
    pub hit_points: u16,
    pub size: Size,
}

impl TableItem for Relic {
    fn id(&self) -> Id {
        self.raw.id
    }
}
