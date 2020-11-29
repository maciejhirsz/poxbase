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

import { Page, Icon, AbilityTable, Spinner } from '../components';
import { db, Id } from '../db';

interface Params {
  id?: string;
}

export const AbilityPage = withRouter(class extends React.Component<RouteComponentProps<Params>> {
  id(): Id {
    return Number(this.props.match.params.id) | 0;
  }

  render() {
    const abilityGroup = db.getAbilityGroup(this.id());

    console.log('Group?', abilityGroup);

    if (!abilityGroup) {
      return <Page><Spinner /></Page>;
    }

    return (
      <Page>
        <h2><span>Ability <Icon kind="chevron-right" /></span> {abilityGroup.name}</h2>

        <div className="Page-main">
          <div className="Page-column-main">
            <AbilityTable
              abilities={abilityGroup.ranks.map(id => db.getAbilityUnchecked(id))}
            />
          </div>
        </div>
      </Page>
    );
  }
}) as React.ComponentClass<{}>;
