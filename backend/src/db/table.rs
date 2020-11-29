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

use std::num::NonZeroU32;
use std::fmt;

use crate::types::Id;

pub struct Table<T> {
    index: Vec<Option<NonZeroU32>>,
    items: Vec<T>,
}

impl<T> fmt::Debug for Table<T>
where
    T: fmt::Debug + TableItem,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.items.iter().map(|entry| (entry.id(), entry)))
            .finish()
    }
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Table {
            index: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl<T> Table<T> {
    pub fn entry<I>(&mut self, item: I) -> &mut T
    where
        I: TableItem + Into<T>,
    {
        let id = item.id() as usize;
        if id >= self.index.len() {
            let diff = 1 + id - self.index.len();

            self.index.reserve(diff);
            self.index.extend(std::iter::repeat(None).take(diff));
        }

        let slot = match &mut self.index[item.id() as usize] {
            Some(slot) => slot.get() as usize,
            empty @ None => {
                self.items.push(item.into());
                let slot = self.items.len();
                *empty = NonZeroU32::new(slot as u32);
                slot
            }
        };

        &mut self.items[slot - 1]
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        if let Some(Some(slot)) = self.index.get(id as usize) {
            return self.items.get((slot.get() as usize) - 1);
        }

        None
    }

    pub fn at(&self, slot: usize) -> Option<&T> {
        self.items.get(slot)
    }

    pub fn at_mut(&mut self, slot: usize) -> Option<&mut T> {
        self.items.get_mut(slot)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}


pub trait TableItem {
    fn id(&self) -> Id;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table() {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        struct Item {
            id: Id,
            name: &'static str,
        }

        impl TableItem for Item {
            fn id(&self) -> Id {
                self.id
            }
        }

        let five = Item {
            id: 5,
            name: "Five",
        };
        let nine = Item {
            id: 9,
            name: "Nine",
        };
        let mut table = Table::default();

        assert_eq!(table.entry(nine), &nine);
        assert_eq!(table.entry(five), &five);

        assert_eq!(table.get(0), None);
        assert_eq!(table.get(5), Some(&five));
        assert_eq!(table.get(7), None);
        assert_eq!(table.get(9), Some(&nine));
        assert_eq!(table.get(10), None);

        assert_eq!(table.index.len(), 10);
        assert_eq!(table.items.len(), 2);
    }
}
