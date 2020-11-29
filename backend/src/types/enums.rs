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
use serde_repr::Serialize_repr;

use crate::types::Id;
use crate::db::{TableItem, EnumId};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Size {
    #[serde(rename = "1x1")]
    OneByOne,
    #[serde(rename = "2x2")]
    TwoByTwo,
}

// TODO: Do a custom Deserialize that handles both strings and ints
#[derive(Serialize_repr, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Rarity {
    #[serde(rename = "COMMON")]
    Common,
    #[serde(rename = "UNCOMMON")]
    Uncommon,
    #[serde(rename = "RARE")]
    Rare,
    #[serde(rename = "EXOTIC")]
    Exotic,
    #[serde(rename = "LIMITED")]
    Limited,
    #[serde(rename = "LEGENDARY")]
    Legendary,
}

// TODO: Do a custom Deserialize that handles both strings and ints
#[derive(Serialize_repr, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Faction {
    #[serde(rename = "Savage Tundra")]
    SavageTundra = 0,
    #[serde(rename = "Ironfist Stronghold")]
    IronfistStronghold = 1,
    #[serde(rename = "K'thir Forest")]
    KthirForest = 2,
    #[serde(rename = "Forglar Swamp")]
    ForglarSwamp = 3,
    #[serde(rename = "Shattered Peaks")]
    ShatteredPeaks = 4,
    #[serde(rename = "Sundered Lands")]
    SunderedLands = 5,
    #[serde(rename = "Underdepths")]
    Underdepths = 6,
    #[serde(rename = "Forsaken Wastes")]
    ForsakenWastes = 7,
}

impl EnumId for Faction {
    const SIZE: usize = 8;

    fn from_id(id: Id) -> Option<Faction> {
        match id {
            0 => Some(Faction::SavageTundra),
            1 => Some(Faction::IronfistStronghold),
            2 => Some(Faction::KthirForest),
            3 => Some(Faction::ForglarSwamp),
            4 => Some(Faction::ShatteredPeaks),
            5 => Some(Faction::SunderedLands),
            6 => Some(Faction::Underdepths),
            7 => Some(Faction::ForsakenWastes),
            _ => None
        }
    }
}

impl TableItem for Faction {
    fn id(&self) -> Id {
        *self as Id
    }
}

impl From<Faction> for Id {
    fn from(faction: Faction) -> Id {
        faction as Id
    }
}
