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

import { db, Id, Rune, Champion } from '../db';

import './RuneDetails.css';

interface Props {
  rune: Rune | Champion;
}

interface Shim {
  id: Id;
  name: string;
}

function renderChampionDetails(champ: Champion): React.ReactNode {
  const races = champ.races.map(id => db.getRaceUnchecked(id));
  const classes = champ.classes.map(id => db.getClassUnchecked(id));

  const list = (items: Shim[], kind: string) => {
    const list: React.ReactNode[] = [];

    items.forEach(({ id, name }, i) => {
      if (i !== 0) {
        list.push(', ');
      }

      list.push(<Link key={id} to={`/${kind}/${id}`}>{name}</Link>);
    });

    return list;
  }

  const raceLabel = races.length === 1 ? 'Race:' : 'Races:';
  const classLabel = classes.length === 1 ? 'Class:' : 'Classes:';

  return (
    <div className="RuneDetails-block">
      <div className="RuneDetails-row">
        <span className="RuneDetails-label">{raceLabel}</span>
        {list(races, 'race')}
      </div>
      <div className="RuneDetails-row">
        <span className="RuneDetails-label">{classLabel}</span>
        {list(classes, 'class')}
      </div>
    </div>
  );
}

export function RuneDetails(props: Props) {
  const { rune } = props;
  const { expansion, artist } = rune;

  const expansionName = db.getExpansionUnchecked(expansion).name;
  const artistName = db.getArtistUnchecked(artist).name;

  const champ = 'races' in rune ? renderChampionDetails(rune) : null;

  return (
    <div className="RuneDetails">
      {champ}
      <div className="RuneDetails-block">
        <div className="RuneDetails-row">
          <span className="RuneDetails-label">Artist:</span>
          <Link to={`/artist/${artist}`}>{artistName}</Link>
        </div>
        <div className="RuneDetails-row">
          <span className="RuneDetails-label">Expansion:</span>
          <Link to={`/expansion/${expansion}`}>{expansionName}</Link>
        </div>
      </div>
    </div>
  );
}

// import './Button.css';

// interface Props {
//   children: React.ReactNode;
//   variant?: 'primary';
//   append?: boolean;
//   prepend?: boolean;
//   onClick?: (event: React.MouseEvent) => void;
// }

// export function Button(props: Props) {
//   const { children, onClick, variant, append, prepend } = props;

//   let className = 'Button';

//   if (variant) className += ` Button-${variant}`;
//   if (append) className += ' Button-append';
//   if (prepend) className += ' Button-prepend';

//   return <button className={className} onClick={onClick}>{children}</button>;
// }
