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
import {
  BrowserRouter as Router,
  Switch,
  Route,
  // useRouteMatch,
  useParams
} from "react-router-dom";

import { db, Id } from './db';
import { Header, Spinner, Page } from './components';
import { ChampionPage, SpellPage, EquipPage, RelicPage, AbilityPage } from './pages';
import { Footer } from './Footer';

import './App.css';

export default class App extends React.Component {
  componentDidMount() {
    db.bind(this);
  }

  componentWillUnmount() {
    db.unbind();
  }

  render() {
    if (!db.ready) {
      return (
        <div className="App">
          <Spinner size={80} />
          <Footer />
        </div>
      );
    }

    return (
      <div className="App">
        <Router>
          <Switch>
            <Route path="/champion/:id">
              <Header />
              <ChampionPage />
            </Route>
            <Route path="/spell/:id">
              <Header />
              <SpellPage />
            </Route>
            <Route path="/equip/:id">
              <Header />
              <EquipPage />
            </Route>
            <Route path="/relic/:id">
              <Header />
              <RelicPage />
            </Route>
            <Route path="/ability/:id">
              <Header />
              <AbilityPage />
            </Route>
            <Route path="/expansion/:id">
              <Header />
              <Expansion />
            </Route>
            <Route path="/faction/:id">
              <Header />
              <Faction />
            </Route>
            <Route path="/">
              <Header home />
            </Route>
          </Switch>
        </Router>
        <Footer />
      </div>
    );
  }
}

interface Params {
  id?: string;
}

function Expansion() {
  const params: Params = useParams();

  console.log(params);

  return <Page>Expansion</Page>;
}

function Faction() {
  const params: Params = useParams();

  console.log(params);

  return <Page>Faction {params.id}</Page>;
}
