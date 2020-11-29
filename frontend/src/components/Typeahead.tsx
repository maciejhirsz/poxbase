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
import { History } from 'history';

import './Typeahead.css';

import { Icon, Button, Input, Dropdown, Label } from './';
import { api, display, hide, isVisible } from '../util';
import { Id, Rarity } from '../db';

interface Response {
  results: Result[];
}

interface Props {
  history: History;
}

interface State extends Response {
  selected: number;
}

interface ResultRune {
  id: Id,
  name: string,
  kind: 'champion' | 'spell' | 'relic' | 'equip' | 'ability' | 'race',
  rarity: Rarity | null,
}

interface ResultEffect {
  id: string,
  name: string,
  kind: 'effect' | 'condition' | 'damage',
  rarity: null,
}

type Result = ResultRune | ResultEffect

const KIND_TO_LABEL = {
  champion: ['champion', 'Champion'],
  spell: ['spell', 'Spell'],
  relic: ['relic', 'Relic'],
  equip: ['equip', 'Equipment'],
  ability: ['ability', 'Ability'],
  race: ['race','Race'],
  effect: ['effect', 'Effect'],
  condition: ['effect', 'Condition'],
  damage: ['effect', 'Damage Type'],
};

export class Typeahead extends React.Component<Props, State> {
  private el: HTMLElement | null = null;
  private query = '';

  state: Readonly<State> = {
    results: [],
    selected: -1,
  };

  shouldComponentUpdate(_: Props, nextState: State) {
    return (
      this.state.results !== nextState.results ||
      this.state.selected !== nextState.selected
    );
  }

  render() {
    const { results, selected } = this.state;
    const { query } = this;

    let qwords = query
      .toLowerCase()
      .replace(/[^a-z\s]/g, '')
      .match(/\S+/g);

    if (!qwords) {
      qwords = [];
    }

    qwords = qwords.map(qword => '\\b' + qword.split('').join('[^a-z\\s]*'));
    const re = new RegExp(qwords.join('|'), 'i');

    const dropdown = results.map(({ id, kind, name, rarity }, i) => {
      const [category, label] = KIND_TO_LABEL[kind];

      let className;

      if (selected === i) {
        className = 'active';
      }

      const highlighted = [];

      let remaining = name;
      let match;

      while (match = remaining.match(re)) {
        if (match.index == null) {
          throw new Error('Invalid regex match');
        }

        const { index, length } = match;
        const [ text ] = match;

        highlighted.push(remaining.substr(0, index));
        highlighted.push(<strong key={remaining.length}>{text}</strong>);

        remaining = remaining.substr(index + text.length);
      }

      highlighted.push(remaining);

      return (
        <Link className={className} key={i} to={`/${category}/${id}`}>
          {highlighted}
          <Label float variant={rarity}>{label}</Label>
        </Link>
      );
    });

    return (
      <div className="Header-search Typeahead">
        <Input prepend type={this.type} onClick={this.show} />
        <Button variant="primary" append>
          <Icon className="Header-search-icon" kind="search" />
        </Button>
        <Dropdown.Menu el={this.setEl} className="Typeahead-menu">
          {dropdown}
        </Dropdown.Menu>
      </div>
    )
  }

  private search() {
    const { results, selected } = this.state;
    const active = results[selected];

    if (active) {
      const { history } = this.props;
      const { id, kind } = active;

      history.push(`/${kind}/${id}`);
    }
  }

  private type = async (event: React.KeyboardEvent) => {
    const { results, selected } = this.state;
    const input = event.target as HTMLInputElement;

    let shift = 0;

    switch (event.key) {
      case 'Escape':
        this.el && hide(this.el);
        this.setState({ selected: -1 });
        return;
      case 'ArrowDown':
        shift = 1;
        break;
      case 'ArrowUp':
        shift = -1;
        break;
      case 'Enter':
        this.el && hide(this.el);
        this.search();
        return;
    }

    if (shift !== 0) {
      event.preventDefault();

      if (!isVisible(this.el)) {
        this.show();
        return;
      }

      let sel = selected + shift;

      if (sel >= results.length) {
        sel = -1;
      } else if (sel < -1) {
        sel = results.length - 1;
      }

      this.setState({ selected: sel });
      return;
    }

    setTimeout(async () => {
      if (this.query === input.value) {
        // Nothing to do
        return;
      }

      const query = this.query = input.value;

      if (query.length === 0) {
        this.el && hide(this.el);
        return;
      }

      let { results }: Response = await api(`/typeahead/${this.query}`);

      if (this.query !== query) {
        // New request was fired while we waited, ditch the resul
        return;
      }

      this.setState({
        results,
        selected: -1,
      });

      if (results.length !== 0) {
        this.el && display(this.el, 'flex');
      } else {
        this.el && hide(this.el);
      }
    }, 50);
  }

  private show = (event?: React.MouseEvent) => {
    if (this.state.results.length !== 0 && this.el) {
      display(this.el, 'flex');
      event?.stopPropagation();
    }
  }

  private setEl = (el: HTMLElement | null) => {
    this.el = el;
  }
}
