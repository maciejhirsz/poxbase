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

import { db, Relic } from '../../db';
import { AbstractRune } from './';

export class RelicRune extends AbstractRune<Relic> {
  protected readonly hasIdol = true;
  protected readonly backLabel = "Relic";

  protected getRune(): Readonly<Relic> | null {
    return db.getRelic(this.props.id);
  }

  protected getFluff(relic: Readonly<Relic>): string {
    return relic.flavorText;
  }

  protected getStats(relic: Readonly<Relic>): React.ReactNode | null {
    const { defense, hitPoints} = relic;

    return (
      <>
        <div className="Rune-stats" />
        <div className="Rune-stat Rune-def">{defense}</div>
        <div className="Rune-stat Rune-hp">{hitPoints}</div>
      </>
    );
  }

  protected getBackprops(relic: Readonly<Relic>): React.ReactNode {
    const { deckLimit, size } = relic;

    return (
      <div className="Rune-back-props">
        <p>Deck Limit: <span>{deckLimit}</span></p>
        <p>Size: <span>{size}</span></p>
      </div>
    );
  }
}
