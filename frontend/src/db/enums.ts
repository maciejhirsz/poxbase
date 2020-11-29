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

import { Id } from './';

export interface RarityShim {
  label: string,
}

export interface Faction {
  short: string,
  name: string,
}

export const RARITY: Readonly<RarityShim[]> = [
  { label: "Common" },
  { label: "Uncommon" },
  { label: "Rare" },
  { label: "Exotic" },
  { label: "Legendary" },
  { label: "Limited" },
];

export const FACTIONS: Readonly<Map<Id, Faction>> = new Map([
  [0, { short: "st", name: "Savage Tundra" }],
  [1, { short: "is", name: "Ironfist Stronghold" }],
  [2, { short: "kf", name: "K'thir Forest" }],
  [3, { short: "fs", name: "Forglar Swamp" }],
  [4, { short: "sp", name: "Shattered Peaks" }],
  [5, { short: "sl", name: "Sundered Lands" }],
  [6, { short: "ud", name: "Underdepths" }],
  [7, { short: "fw", name: "Forsaken Wastes" }],
]);
