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

use crate::types::Id;
use crate::db::TableItem;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbilityCore {
    pub id: Id,
    pub ap_cost: u8,
    pub name: Box<str>,
    pub short_description: Box<str>,
    pub activation_type: u8,
    pub level: u8,
    pub cooldown: u8,
    pub nora_cost: i8,
    pub icon_name: Box<str>,
}

#[derive(Serialize, Debug)]
pub struct Ability {
    #[serde(flatten)]
    pub core: AbilityCore,
    pub group: Id,
}

impl From<AbilityCore> for Ability {
    fn from(core: AbilityCore) -> Self {
        Ability {
            core,
            group: !0,
        }
    }
}

impl TableItem for Ability {
    fn id(&self) -> Id {
        self.core.id
    }
}

impl TableItem for AbilityCore {
    fn id(&self) -> Id {
        self.id
    }
}
