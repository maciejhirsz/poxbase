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

import { db, Id, Champion } from '../../db';
import { AbstractRune } from './';

interface Props {
  id: Id;
  selected: [Id, Id];
  nora?: number;
}

export class ChampionRune extends AbstractRune<Champion, Props> {
  protected readonly hasIdol = true;
  protected readonly backLabel = "Champion";

  protected getRune(): Readonly<Champion> | null {
    return db.getChampion(this.props.id);
  }

  protected getNora(champion: Readonly<Champion>): number {
    return this.props.nora || champion.noraCost;
  }

  protected getFluff(champion: Readonly<Champion>): string {
    return champion.description;
  }

  protected getStats(champion: Readonly<Champion>): React.ReactNode | null {
    const { damage, speed, minRng, maxRng, defense, hitPoints} = champion;

    return (
      <>
        <div className="Rune-stats" />
        <div className="Rune-stat Rune-dmg">{damage}</div>
        <div className="Rune-stat Rune-spd">{speed}</div>
        <div className="Rune-stat Rune-rng">{minRng} - {maxRng}</div>
        <div className="Rune-stat Rune-def">{defense}</div>
        <div className="Rune-stat Rune-hp">{hitPoints}</div>
      </>
    );
  }

  protected getBacksideCenter(champion: Readonly<Champion>): React.ReactNode {
    const abilities = this.props.selected
      .concat(champion.startingAbilities)
      .map(id => {
        const ability = db.getAbilityUnchecked(id);
        const active = ability.activationType ? 'active_' : '';
        const name = ability.level ? `${ability.name} (${ability.level})` : ability.name;

        return (
          <div key={id} className="Rune-ab">
            <div className="Rune-ab-icon" style={{ backgroundImage: `url('/assets/big_icons/icon_${ability.iconName}.png')` }} />
            <img src={`/assets/frame/ability_${active}border.png`} className="Rune-ability-border" />
            <span>{name}</span>
          </div>
        );
      });

    const level = this.props.selected[1] !== champion.defaults[1]
      ? '3'
      : this.props.selected[0] !== champion.defaults[0]
      ? '2'
      : '1';

    return (
      <>
        <div className="Rune-level">{level}</div>
        <div className="Rune-abilities">
          <div className="Rune-abilities-col-1">{abilities.slice(0, 4)}</div>
          <div className="Rune-abilities-col-2">{abilities.slice(4, 8)}</div>
        </div>
      </>
    );
  }

  protected getBackprops(champion: Readonly<Champion>): React.ReactNode {
    const { deckLimit, size } = champion;
    const races = db.getRaces(champion).join(', ');
    const classes = db.getClasses(champion).join(', ');

    return (
      <div className="Rune-back-props">
        <p>Deck Limit: <span>{deckLimit}</span></p>
        <p>Races: <span>{races}</span></p>
        <p>Classes: <span>{classes}</span></p>
        <p>Size: <span>{size}</span></p>
      </div>
    );
  }
}
