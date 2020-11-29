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
use arrayvec::ArrayVec;

use crate::types::{Id, EntityId, RuneRaw, Size, Faction};
use crate::db::TableItem;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionCore {
    #[serde(flatten)]
    pub raw: RuneRaw,
    pub max_rng: u8,
    pub min_rng: u8,
    pub defense: u8,
    pub speed: u8,
    pub damage: u16,
    pub hit_points: u16,
    pub size: Size,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    #[serde(flatten)]
    pub core: ChampionCore,
    pub starting_abilities: ArrayVec<[Id; 8]>,
    pub ability_sets: [ArrayVec<[Id; 4]>; 2],
    pub defaults: [Id; 2],
    pub classes: ArrayVec<[Id; 6]>,
    pub races: ArrayVec<[Id; 6]>,
    pub factions: ArrayVec<[Faction; 2]>,
    pub expansion: Id,
    pub artist: Id,
}

impl From<ChampionCore> for Champion {
    fn from(core: ChampionCore) -> Self {
        Champion {
            core,
            starting_abilities: ArrayVec::new(),
            ability_sets: [ArrayVec::new(), ArrayVec::new()],
            defaults: [!0, !0],
            classes: ArrayVec::new(),
            races: ArrayVec::new(),
            factions: ArrayVec::new(),
            expansion: !0,
            artist: !0,
        }
    }
}

impl From<&Champion> for EntityId {
    fn from(taggable: &Champion) -> EntityId {
        EntityId::Champion(taggable.id())
    }
}

impl TableItem for Champion {
    fn id(&self) -> Id {
        self.core.raw.id
    }
}

impl TableItem for ChampionCore {
    fn id(&self) -> Id {
        self.raw.id
    }
}
