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

import { Rarity } from '../db';

import './Label.css';

interface Props {
  children: React.ReactNode;
  float?: boolean;
  title?: string;
  variant?: 'nora' | Rarity | null;
}

export function Label(props: Props) {
  const { children, variant, title, float } = props;

  let className = 'Label';

  if (variant != null) {
    className += ` Label-${variant}`;
  }

  if (float) {
    className += ` Label-float`;
  }

  return (
    <span className={className} title={title}>
      {children}
    </span>
  );
}
