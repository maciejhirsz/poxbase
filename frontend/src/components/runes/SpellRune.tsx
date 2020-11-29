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

import { db, Spell } from '../../db';
import { AbstractRune } from './';

export class SpellRune extends AbstractRune<Spell> {
  protected readonly backLabel = "Spell";

  protected getRune(): Readonly<Spell> | null {
    return db.getSpell(this.props.id);
  }

  protected getFluff(spell: Readonly<Spell>): string {
    return spell.flavorText;
  }
}
