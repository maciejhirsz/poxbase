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
import { Link } from 'react-router-dom';

import { Ability, Id } from '../db';
import { AbilityIcon, Label, GameText } from '.';

import './AbilityTable.css';

interface Props {
  abilities: Ability[];
  selected?: Id;
  onSelect?: (id: Id) => void;
}

export class AbilityTable extends React.Component<Props> {
  shouldComponentUpdate(nextProps: Props) {
    return (
      this.props.abilities !== nextProps.abilities ||
      this.props.selected !== nextProps.selected
    );
  }

  render() {
    const { selected, onSelect } = this.props;

    const rows = this.props.abilities.map(ability => {
      const { id, name, group, apCost, level, cooldown, noraCost, shortDescription } = ability;
      const href = `/ability/${group}`;
      const onClick = onSelect && (() => onSelect(id));

      let rowClass = 'AbilityTable-row';
      let icon = <AbilityIcon ability={ability} />;

      if (id === selected) {
        rowClass += ' AbilityTable-row-selected';
      }

      if (selected) {
        rowClass += ' AbilityTable-row-selectable';
      } else {
        icon = <Link to={href}>{icon}</Link>;
      }

      const rank = level ? <span className="AbilityTable-rank">{level}</span> : null;
      const apLabel = apCost ? <Label float title="Action Point Cost">AP: {apCost}</Label> : null;
      const cdLabel = cooldown ? <Label float title="Cooldown">CD: {cooldown}</Label> : null;

      return (
        <div key={id} className={rowClass} onClick={onClick}>
          <div className="AbilityTable-icon">
            {icon}
          </div>
          <div>
            <Link to={href} className="AbilityTable-name">{name} {rank}</Link>
            <Label float variant="nora">{noraCost} Nora</Label>
            {apLabel}
            {cdLabel}
            <p>
              <GameText>{shortDescription}</GameText>
            </p>
          </div>
        </div>
      );
    });

    return (
      <div className="AbilityTable">
        {rows}
      </div>
    );
  }
}
