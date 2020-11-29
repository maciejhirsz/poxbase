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

import React from 'react';
import { Id, Champion, Equip, Spell, Relic, Ability, AbilityGroup, Race, Class, Expansion, Artist } from './';
import { api } from '../util';

interface Response {
  expansions?: Expansion[];
  champs?: Champion[];
  spells?: Spell[];
  equips?: Equip[];
  relics?: Relic[];
  abilities?: Ability[];
  abilityGroups?: AbilityGroup[];
  races?: Race[];
  classes?: Class[];
  artists?: Artist[];
}

interface HasId {
  id: Id;
}

type Table<T> = Map<Id, T | null>;

function apply<T extends HasId>(table: Table<T>, data?: T[]) {
  if (!data) return;

  for (const item of data) {
    const existing = table.get(item.id);

    if (existing) {
      Object.assign(existing, item);
    } else {
      table.set(item.id, item);
    }
  }
}

export default class DB {
  private bound: React.Component | null = null;

  private _ready = false;
  private _expansions: Expansion[] = [];
  private champions: Table<Champion> = new Map();
  private spells: Table<Spell> = new Map();
  private equips: Table<Equip> = new Map();
  private relics: Table<Relic> = new Map();
  private races: Table<Race> = new Map();
  private classes: Table<Class> = new Map();
  private abilities: Table<Ability> = new Map();
  private abilityGroups: Table<AbilityGroup> = new Map();
  private artists: Table<Artist> = new Map();

  constructor() {
    this.fetch('/init', (res: Response) => {
      if (res.expansions != null) {
        this._expansions = res.expansions;
        this._ready = true;
      }
    });
  }

  public getChampion(id: Id): Readonly<Champion> | null {
    const champion = this.champions.get(id);

    if (champion === undefined) {
      this.champions.set(id, null);

      this.fetch(`/champ/${id}`, (res: Response) => {
        apply(this.champions, res.champs);
        apply(this.races, res.races);
        apply(this.classes, res.classes);
        apply(this.abilities, res.abilities);
        apply(this.artists, res.artists);
      });

      return null;
    }

    return champion;
  }

  public getSpell(id: Id): Readonly<Spell> | null {
    const spell = this.spells.get(id);

    if (spell === undefined) {
      this.spells.set(id, null);

      this.fetch(`/spell/${id}`, (res: Response) => {
        apply(this.spells, res.spells);
        apply(this.artists, res.artists);
      });

      return null;
    }

    return spell;
  }

  public getEquip(id: Id): Readonly<Equip> | null {
    const equip = this.equips.get(id);

    if (equip === undefined) {
      this.equips.set(id, null);

      this.fetch(`/equip/${id}`, (res: Response) => {
        apply(this.equips, res.equips);
        apply(this.artists, res.artists);
      });

      return null;
    }

    return equip;
  }

  public getRelic(id: Id): Readonly<Relic> | null {
    const relic = this.relics.get(id);

    if (relic === undefined) {
      this.relics.set(id, null);

      this.fetch(`/relic/${id}`, (res: Response) => {
        apply(this.relics, res.relics);
        apply(this.artists, res.artists);
      });

      return null;
    }

    return relic;
  }

  public getAbilityGroup(id: Id): Readonly<AbilityGroup> | null {
    const abilityGroup = this.abilityGroups.get(id);

    if (abilityGroup === undefined) {
      this.abilityGroups.set(id, null);

      this.fetch(`/ability/${id}`, (res: Response) => {
        apply(this.abilityGroups, res.abilityGroups);
        apply(this.abilities, res.abilities);
      });

      return null;
    }

    return abilityGroup;
  }

  public getClasses(champion: Champion): string[] {
    return champion.classes.map(id => this.getClassUnchecked(id).name);
  }

  public getRaces(champion: Champion): string[] {
    return champion.races.map(id => this.getRaceUnchecked(id).name);
  }

  public getAbilityUnchecked(id: Id): Readonly<Ability> {
    return this.abilities.get(id) as Ability;
  }

  public getRaceUnchecked(id: Id): Readonly<Race> {
    return this.races.get(id) as Race;
  }

  public getClassUnchecked(id: Id): Readonly<Class> {
    return this.classes.get(id) as Class;
  }

  public getExpansionUnchecked(id: Id): Readonly<Expansion> {
    return this._expansions[id];
  }

  public getArtistUnchecked(id: Id): Readonly<Artist> {
    return this.artists.get(id) as Artist;
  }

  public get expansions(): Readonly<Expansion[]> {
    return this._expansions;
  }

  public get ready(): boolean {
    return this._ready;
  }

  public bind(component: React.Component) {
    if (this.bound) {
      throw new Error("DB can be only bound to one component");
    }

    this.bound = component;
  }

  public unbind() {
    this.bound = null;
  }

  private async fetch(path: string, cb: (res: Response) => void) {
    cb(await api(path));

    this.bound?.forceUpdate();
  }
}
