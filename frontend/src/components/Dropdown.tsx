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

import { Icon, Button } from './';
import { toggleDisplay } from '../util';

import './Dropdown.css';

interface Props {
  label: string;
  children: React.ReactNode[];
  wrap?: number;
}

interface MenuProps {
  children: React.ReactNode[];
  className?: string;
  wrap?: number;
  el: (el: HTMLElement | null) => void;
}

function renderColumns(children: React.ReactNode[], wrap: number): React.ReactNode[] {
    const columns = [];

    for (let i = 0; i < children.length; i += wrap) {
      columns.push(
        <div key={i} className="Dropdown-column">
          {children.slice(i, i + wrap)}
        </div>
      );
    }

    return columns;
  }

export class Dropdown extends React.Component<Props> {
  public static readonly Divider = () => <hr className="Dropdown-divider" />;
  public static readonly Menu = (props: MenuProps) => {
    const { el, children, wrap, className } = props;

    let menuClass = className ? `Dropdown-menu ${className}` : 'Dropdown-menu';


    const columns = wrap ? renderColumns(children, wrap) : (
      <div className="Dropdown-column">
        {children}
      </div>
    );

    return (
      <div className={menuClass} ref={el}>
        {columns}
      </div>
    )
  }

  private el: HTMLElement | null = null;

  render() {
    const { label, children, wrap } = this.props;

    return (
      <div className="Dropdown">
        <Button onClick={this.toggle}>{label} <Icon kind="caret-down" /></Button>

        <Dropdown.Menu el={this.setEl} wrap={wrap}>
          {children}
        </Dropdown.Menu>
      </div>
    )
  }

  private setEl = (el: HTMLElement | null) => {
    this.el = el;
  }

  private toggle = (event: React.MouseEvent) => {
    event.stopPropagation();

    this.el && toggleDisplay(this.el, 'inline-flex');
  }
}
