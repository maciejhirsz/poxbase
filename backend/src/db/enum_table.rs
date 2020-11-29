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

use crate::db::{Id, TableItem};
use crate::types::RuneSet;

#[derive(Debug)]
pub struct EnumTable<E: EnumId> {
    table: Vec<RuneSet<E>>
}

pub trait EnumId: TableItem + Copy {
    const SIZE: usize;

    fn from_id(id: Id) -> Option<Self>;
}

impl<E> Default for EnumTable<E>
where
    E: EnumId,
{
    fn default() -> Self {
        let mut table = Vec::with_capacity(E::SIZE);
        let mut id = 0;

        while let Some(key) = E::from_id(id) {
            table.push(RuneSet::from(key));
            id += 1;
        }

        EnumTable {
            table
        }
    }
}

impl<E> EnumTable<E>
where
    E: EnumId,
{
    pub fn entry(&mut self, item: E) -> &mut RuneSet<E> {
        &mut self.table[item.id() as usize]
    }
}
