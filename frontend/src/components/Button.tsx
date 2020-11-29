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

import './Button.css';

interface Props {
  children: React.ReactNode;
  variant?: 'primary';
  append?: boolean;
  prepend?: boolean;
  onClick?: (event: React.MouseEvent) => void;
}

export function Button(props: Props) {
  const { children, onClick, variant, append, prepend } = props;

  let className = 'Button';

  if (variant) className += ` Button-${variant}`;
  if (append) className += ' Button-append';
  if (prepend) className += ' Button-prepend';

  return <button className={className} onClick={onClick}>{children}</button>;
}
