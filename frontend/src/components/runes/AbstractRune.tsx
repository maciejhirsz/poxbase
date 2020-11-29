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

import { db, Id, Rune, FACTIONS } from '../../db';
import { Spinner, GameText, Blink } from '../';

import './Rune.css';

interface Props {
  id: Id;
}

interface State {
  flip: boolean;
}

export abstract class AbstractRune<T extends Rune, P extends Props = Props> extends React.Component<P, State> {
  state = { flip: false };

  protected readonly hasIdol: boolean = false;
  protected abstract readonly backLabel: string = '';
  protected abstract getRune(): Readonly<T> | null;
  protected abstract getFluff(rune: Readonly<T>): string;

  protected getNora(rune: Readonly<T>): number {
    return rune.noraCost;
  }

  protected getStats(rune: Readonly<Rune>): React.ReactNode | null {
    return null;
  }

  protected getBacksideCenter(rune: Readonly<Rune>): React.ReactNode {
    return (
      <div className="Rune-description">
        <GameText>{rune.description}</GameText>
      </div>
    );
  }

  protected getBackprops(rune: Readonly<T>): React.ReactNode {
    return (
      <div className="Rune-back-props">
        <p>Deck Limit: <span>{rune.deckLimit}</span></p>
      </div>
    );
  }

  render() {
    const rune = this.getRune();
    const { flip } = this.state;
    const className = flip ? 'Rune Rune-flip' : 'Rune';

    if (!rune) {
      return (
        <div className={className} onClick={this.flip}>
          <div className="Rune-card Rune-card-front">
            <Spinner />
          </div>
          <div className="Rune-card Rune-card-reverse">
            <Spinner />
          </div>
        </div>
      );
    }

    const { id, hash, name, rarity, description, deckLimit } = rune;
    const factions = rune.factions.map(id => FACTIONS.get(id)?.short);
    const factionA = factions[0];
    const factionB = factions[1] || factions[0];
    const frameCode = factions.join('_');
    const idol = this.hasIdol
      ?
        (
          <div className="Rune-idol">
            <div className="Rune-idol-wrapper">
              <img src={`/assets/idols/${hash}.gif`} />
            </div>
          </div>
        )
      : null;

    return (
      <div className={className} onClick={this.flip}>
        <div className="Rune-card Rune-card-front">
          <div className="Rune-label">{name}</div>
          <Blink className="Rune-nora" id={id}>{this.getNora(rune)}</Blink>
          <img src={`/assets/runes/${hash}.jpg`} className="Rune-picture" />
          <img src={`/assets/frame/rarity_${rarity}.gif`} className="Rune-rarity" />
          <img src={`/assets/frame/front/${frameCode}.gif`} className="Rune-frame" />
          <img src={`/assets/frame/icon/${factionA}_1.gif`} className="Rune-faction-icon" />
          <img src={`/assets/frame/icon/${factionB}_2.gif`} className="Rune-faction-icon" />
          {this.getStats(rune)}
        </div>
        <div className="Rune-card Rune-card-reverse">
          <div className="Rune-label">{this.backLabel}</div>
          <img src={`/assets/frame/back/${frameCode}.gif`} className="Rune-frame" />
          <img src={`/assets/frame/icon/${factionA}_1.gif`} className="Rune-faction-icon" />
          <img src={`/assets/frame/icon/${factionB}_2.gif`} className="Rune-faction-icon" />
          <div className="Rune-fluff">{this.getFluff(rune)}</div>
          {this.getBacksideCenter(rune)}
          {this.getBackprops(rune)}
          {idol}
        </div>
      </div>
    )
  }

  private flip = () => {
    this.setState({ flip: !this.state.flip });
  };
}
