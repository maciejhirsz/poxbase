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

use std::cmp::Ordering;
use arrayvec::ArrayString;
use serde::Serialize;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry::*;

use crate::types::{Id, EntityId};

type Word = ArrayString<[u8; 15]>;

#[derive(Serialize, Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", content = "id", rename_all = "camelCase")]
pub enum SearchId<'a> {
    Champion(Id),
    Spell(Id),
    Equip(Id),
    Relic(Id),
    #[serde(rename = "ability")]
    AbilityGroup(Id),
    Effect(&'a str),
    Condition(&'a str),
    Damage(&'a str),
}

pub trait Searchable {
    fn search_id(&self) -> SearchId;
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    eid: EntityId,
    ord: u32,
}

#[derive(Debug)]
pub struct SearchIndex {
    table: Vec<(Word, Entry)>,
}

impl Default for SearchIndex {
    fn default() -> Self {
        SearchIndex {
            table: Vec::new(),
        }
    }
}

fn left_bound<T: Ord>(a: &T, b: &T) -> Ordering {
    if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn right_bound(a: &str, b: &str) -> Ordering {
    if a.starts_with(b) {
        Ordering::Less
    } else {
        a.cmp(b)
    }
}

impl SearchIndex {
    pub fn insert(&mut self, text: &str, eid: EntityId) {
        for (order, word) in Self::split_words(text).enumerate() {
            let entry = Entry {
                eid,
                ord: order as u32,
            };

            let index = self.table.binary_search_by(|(w, _)| w.cmp(&word)).unwrap_or_else(|i| i);

            self.table.insert(index, (word, entry));
        }
    }

    pub fn find(&self, query: &str) -> Vec<(EntityId, i32)> {
        let mut entries = FxHashMap::default();
        let mut sums = FxHashMap::default();
        let mut threshold = 0;

        for (order, word) in Self::split_words(query).take(10).enumerate() {
            let order = order as u32;
            let score = (word.len() * 8) as i32;

            // Ignoring order of things, we must at least find all the letters
            threshold += score;

            let start = self.table.binary_search_by(|(w, _)| left_bound(w, &word)).unwrap_or_else(|i| i);
            let count = self.table[start..].binary_search_by(|(w, _)| right_bound(w, &word)).unwrap_or_else(|i| i);
            let matches = &self.table[start..start + count];

            for (w, entry) in matches {
                // Add a bonus to the score if the word order matches query
                let mut score = score + 7;
                score -= std::cmp::min((w.len() - word.len()) as i32, 4);
                score -= ((order as i32) - (entry.ord as i32)).abs();

                match entries.entry(Entry {
                    eid: entry.eid,
                    ord: order,
                }) {
                    Vacant(vacant) => {
                        vacant.insert(score);
                        *sums.entry(entry.eid).or_insert(0) += score;
                    }
                    Occupied(occupied) => {
                        let s = *occupied.get();

                        if score > s {
                            *sums.entry(entry.eid).or_insert(0) += score - s;
                        }
                    }
                }
            }
        }

        let mut sorted = sums.into_iter().filter(|(_, score)| *score >= threshold).collect::<Vec<_>>();

        sorted.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        sorted
    }

    pub fn split_words(text: &str) -> impl Iterator<Item = Word> + '_ {
        text.split_ascii_whitespace().map(|w| {
            let mut word = Word::new();

            let bytes = w.bytes().filter_map(|b| match b | 0x20 {
                ch @ b'a'..=b'z' => Some(ch),
                _ => None
            });

            for byte in bytes.take(15) {
                word.push(byte as char);
            }

            word
        })
    }

    pub fn size(&self) -> usize {
        self.table.len() * std::mem::size_of::<(Word, Entry)>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_words() {
        let split = SearchIndex::split_words("Hello W'orld!").collect::<Vec<_>>();

        assert_eq!(&split, &[Word::from("hello").unwrap(), Word::from("world").unwrap()]);
    }
}
