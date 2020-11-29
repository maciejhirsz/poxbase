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
import { AbilityIcon, Label } from '.';

interface Props {
  children: string;
}

const re = /\[([^\]]+)\](?:\(([^)]+)\)|\*)/m;

export class GameText extends React.Component<Props> {
  shouldComponentUpdate(nextProps: Props) {
    return this.props.children !== nextProps.children;
  }

  render() {
    const fixed = [];

    let remaining = this.props.children;
    let match;

    while (match = remaining.match(re)) {
      if (match.index == null) {
        throw new Error('Invalid regex match');
      }

      const { index, length } = match;
      const [ m, text, href ] = match;

      fixed.push(remaining.substr(0, index));
      if (href === '*') {
        fixed.push(<strong key={remaining.length}>{text}</strong>);
      } else {
        fixed.push(<Link key={remaining.length} to={href}>{text}</Link>);
      }

      remaining = remaining.substr(index + m.length);
    }

    fixed.push(remaining);

    return fixed;
  }
}
