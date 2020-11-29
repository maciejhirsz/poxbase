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

import React, { useState } from 'react';
import { RouteComponentProps, withRouter } from "react-router-dom";

import { Page, Icon, Spinner, ChampionRune, AbstractRune, AbilityTable, RuneDetails } from '../components';
import { db, Id } from '../db';

interface Params {
  id?: string;
}

export const ChampionPage = withRouter(class extends React.Component<RouteComponentProps<Params>> {
  private selectedFirst: Id | null = null;
  private selectedSecond: Id | null = null;
  private champion = db.getChampion(this.id(this.props));

  componentWillReceiveProps(nextProps: RouteComponentProps<Params>) {
    const nextChampion = db.getChampion(this.id(nextProps));

    if (this.champion !== nextChampion) {
      this.selectedFirst = null;
      this.selectedSecond = null;
      this.champion = nextChampion;
    }
  }

  id(props: RouteComponentProps<Params>): Id {
    return Number(props.match.params.id) | 0;
  }

  render() {
    const { champion } = this;

    if (!champion) {
      return <Page><Spinner /></Page>;
    }

    const first = this.selectedFirst !== null ? this.selectedFirst : champion.defaults[0];
    const second = this.selectedSecond !== null ? this.selectedSecond : champion.defaults[1];

    const nora = champion.noraCost - (
      db.getAbilityUnchecked(champion.defaults[0]).noraCost +
      db.getAbilityUnchecked(champion.defaults[1]).noraCost
    ) + (
      db.getAbilityUnchecked(first).noraCost +
      db.getAbilityUnchecked(second).noraCost
    );

    return (
      <Page>
        <h2><span>Champion <Icon kind="chevron-right" /></span> {champion.name}</h2>

        <div className="Page-main">
          <div className="Page-column-side">
            <ChampionRune id={champion.id} nora={nora} selected={[first, second]} />
            <RuneDetails rune={champion} />
          </div>
          <div className="Page-column-main">
            <h6>Level 2 Ability Choices</h6>
            <AbilityTable
              abilities={champion.abilitySets[0].map(id => db.getAbilityUnchecked(id))}
              selected={first}
              onSelect={this.setFirst}
            />
            <h6>Level 3 Ability Choices</h6>
            <AbilityTable
              abilities={champion.abilitySets[1].map(id => db.getAbilityUnchecked(id))}
              selected={second}
              onSelect={this.setSecond}
            />
            <h6>Base Abilities</h6>
            <AbilityTable abilities={champion.startingAbilities.map(id => db.getAbilityUnchecked(id))} />
          </div>
        </div>
      </Page>
    );
  }

  private setFirst = (id: Id) => {
    if (this.selectedFirst !== id) {
      this.selectedFirst = id;
      this.forceUpdate();
    }
  }

  private setSecond = (id: Id) => {
    if (this.selectedSecond !== id) {
      this.selectedSecond = id;
      this.forceUpdate();
    }
  }
}) as React.ComponentClass<{}>;
