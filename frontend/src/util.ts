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

let displayed: HTMLElement | null = null;

const BACKEND = window.location.protocol === 'http:' ? 'http://localhost:8000' : '/api';

window.addEventListener('click', () => {
  displayed?.style.removeProperty('display');
  displayed = null;
});

export function toggleDisplay(el: HTMLElement, style = 'block') {
  if (el === displayed) {
    el.style.removeProperty('display');
    displayed = null;
    return;
  }

  displayed?.style.removeProperty('display');
  displayed = el;
  el.style.setProperty('display', style);
}

export function display(el: HTMLElement, style = 'block') {
  if (displayed !== el) {
    displayed?.style.removeProperty('display');
    displayed = el;
  }
  el.style.setProperty('display', style);
}

export function hide(el: HTMLElement) {
  el.style.removeProperty('display');

  if (el === displayed) {
    displayed = null;
  }
}

export function isVisible(el: HTMLElement | null) {
  return displayed === el;
}

export async function api<T>(path: string): Promise<T> {
  const req = await fetch(BACKEND + path);

  return await req.json() as T
}
