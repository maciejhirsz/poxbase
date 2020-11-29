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

import './Blink.css';

interface Props {
  id: any;
  children: string | number;
  className?: string;
}

export class Blink extends React.Component<Props> {
  content = this.props.children;
  el: HTMLDivElement | null = null;

  shouldComponentUpdate(nextProps: Props) {
    if (this.props.id !== nextProps.id) {
      this.content = nextProps.children;
      this.el && (this.el.textContent = String(this.content));
    }

    this.blink(nextProps.children);

    return false;
  }

  render() {
    const { className } = this.props;

    const blinkClass = className ? `Blink ${className}` : 'Blink';

    return <div className={blinkClass} ref={this.onRef}>{this.content}</div>;
  }

  private blink(content: string | number) {
    if (this.el && this.content !== content) {
      this.el.style.setProperty('opacity', '0');
      this.content = content;

      setTimeout(() => {
        if (this.el) {
          this.el.textContent = String(this.content);
          this.el.style.setProperty('opacity', '1');
        }
      }, 300);
    }
  }

  private onRef = (el: HTMLDivElement) => {
    this.el = el;
  }
}
