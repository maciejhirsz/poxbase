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
import { Link, useHistory } from 'react-router-dom';

import { db } from '../db';
import { Icon, Dropdown, Button, Typeahead } from './';

import logo from '../img/poxbase-logo.svg';
import './Header.css';

interface Props {
  home?: boolean;
}

export function Header(props: Props) {
  const history = useHistory();
  const className = props.home ? "Header Header-home" : "Header Header-page";

  return (
    <div className={className}>
      <h1 className="Header-h1"><Link to="/"><img src={logo} alt="PoxBase" className="Header-logo" /></Link></h1>

      <div className="Header-toolbar">
        <Typeahead history={history} />

        <nav className="Header-nav">
          <Button><Icon kind="wrench" /></Button>

          <Dropdown label="Expansions" wrap={12}>
            {db.expansions.map((expansion, i) => (
              <Link key={i} to={`/expansion/${expansion.id}`}>{expansion.name}</Link>
            ))}
          </Dropdown>

          <Dropdown label="Factions">
            <Link to="/faction/fs">Forglar Swamp</Link>
            <Link to="/faction/is">Ironfist Stronghold</Link>
            <Link to="/faction/kf">K'thir Forest</Link>
            <Link to="/faction/st">Savage Tundra</Link>
            <Dropdown.Divider />
            <Link to="/faction/fw">Forsaken Wastes</Link>
            <Link to="/faction/sl">Sundered Lands</Link>
            <Link to="/faction/ud">Underdepths</Link>
            <Link to="/faction/sp">Shattered Peaks</Link>
          </Dropdown>

          <Dropdown label="More">
              <Link to="/abilities">Abilities</Link>
              <Link to="/effects">Effects</Link>
              <Link to="/races">Races</Link>
              <Link to="/classes">Classes</Link>
              <Dropdown.Divider />
              <Link to="/artists">Artists</Link>
          </Dropdown>
        </nav>
      </div>
    </div>
  );
}
