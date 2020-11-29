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

export type Id = number;

export type Common = 0;
export type Uncommon = 1;
export type Rare = 2;
export type Exotic = 3;
export type Limited = 4;
export type Legendary = 6;
export type Rarity =
  | Common
  | Uncommon
  | Rare
  | Exotic
  | Limited
  | Legendary;

export interface Rune {
  id: Id,
  name: string,
  description: string,
  hash: string,
  noraCost: number,
  rarity: Rarity,
  factions: Id[],
  deckLimit: number,
  artist: Id,
  expansion: Id,
}

export interface Champion extends Rune {
  damage: number;
  speed: number;
  minRng: number;
  maxRng: number;
  defense: number;
  hitPoints: number;
  size: string;
  races: Id[];
  classes: Id[];
  defaults: Id[];
  abilitySets: [Id[], Id[]];
  startingAbilities: Id[];
}

export interface Spell extends Rune {
  flavorText: string,
}

export interface Equip extends Rune {
  flavorText: string,
}

export interface Relic extends Rune {
  flavorText: string,
  size: string,
  defense: number,
  hitPoints: number,
}

export interface Ability {
  id: Id,
  name: string,
  iconName: string,
  activationType: number,
  shortDescription: string,
  level: number,
  noraCost: number,
  apCost: number,
  cooldown: number,
  group: Id,
}

export interface AbilityGroup {
  id: Id,
  name: string,
  ranks: Id[],
  champs: Id[],
}

export interface Class {
  id: Id,
  name: string,
}

export interface Race {
  id: Id,
  name: string,
}

export interface Expansion {
  id: Id,
  name: string,
}

export interface Artist {
  id: Id,
  name: string,
}

