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

use crate::types::{Id, Ability, AbilityGroup, Champion, Effect, Rune, Spell, Equip, Relic, Group, Faction, RuneSet};

mod table;
mod autoindex;
mod enum_table;
mod search;

pub use table::{Table, TableItem};
pub use autoindex::{AutoIndexTable, AutoIndexItem};
pub use enum_table::{EnumTable, EnumId};
pub use search::{SearchIndex, SearchId, Searchable};

#[derive(Debug, Default)]
pub struct DB {
    pub ability_groups: AutoIndexTable<AbilityGroup>,
    pub abilities: Table<Ability>,
    pub effects: AutoIndexTable<Effect>,
    pub champs: Table<Champion>,
    pub spells: Table<Rune<Spell>>,
    pub equips: Table<Rune<Equip>>,
    pub relics: Table<Rune<Relic>>,
    pub races: AutoIndexTable<Group<Champion>>,
    pub classes: AutoIndexTable<Group<Champion>>,
    pub artists: AutoIndexTable<RuneSet>,
    pub factions: EnumTable<Faction>,
    pub expansions: AutoIndexTable<RuneSet>,
    pub search: SearchIndex,
}
