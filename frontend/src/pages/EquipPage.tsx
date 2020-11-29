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

import { Page, Spinner, EquipRune, RuneDetails, GameText, Icon } from '../components';
import { db, Id } from '../db';

interface Params {
  id?: string;
}

export const EquipPage = withRouter(class extends React.Component<RouteComponentProps<Params>> {
  id(): Id {
    return Number(this.props.match.params.id) | 0;
  }

  render() {
    const equip = db.getEquip(this.id());

    if (!equip) {
      return <Page><Spinner /></Page>;
    }

    return (
      <Page>
        <h2><span>Equipment <Icon kind="chevron-right" /></span> {equip.name}</h2>

        <div className="Page-main">
          <div className="Page-column-side">
            <EquipRune id={equip.id} />
            <RuneDetails rune={equip} />
          </div>
          <div className="Page-column-main">
            <blockquote>
              {equip.flavorText}
            </blockquote>
            <div className="Page-well">
              <GameText>{equip.description}</GameText>
            </div>
          </div>
        </div>
      </Page>
    );
  }
}) as React.ComponentClass<{}>;
