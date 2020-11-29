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

use std::{fs, io};
use std::fmt::Write;
use std::time::Instant;
use arrayvec::ArrayVec;
use serde::Deserialize;
use regex::Regex;
use tiny_keccak::{Keccak, Hasher};

use crate::db::{DB, TableItem, AutoIndexItem};
use crate::types::{Id, EntityId, Faction, AbilityCore, ChampionCore};
use crate::types::{Spell, Equip, Relic, Effect, EffectKind};
// https://www.poxnora.com/api/feed.do?t=json
// https://www.poxnora.com/api/feed.do?t=json&r=mechanics
// https://www.poxnora.com/api/feed.do?t=json&r=conditions

#[derive(Deserialize, Debug)]
struct Feed {
    champs: Vec<FeedChamp>,
    spells: Vec<FeedRune<Spell>>,
    equips: Vec<FeedRune<Equip>>,
    relics: Vec<FeedRune<Relic>>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FeedAbility {
    #[serde(flatten)]
    core: AbilityCore,
    default: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FeedChamp {
    #[serde(flatten)]
    core: ChampionCore,
    classes: ArrayVec<[Box<str>; 6]>,
    races: ArrayVec<[Box<str>; 6]>,
    artist: Box<str>,
    factions: ArrayVec<[Faction; 3]>,
    rune_set: Box<str>,
    starting_abilities: Vec<AbilityCore>,
    ability_sets: [AbilitySet; 2],
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FeedRune<C> {
    #[serde(flatten)]
    core: C,
    artist: Box<str>,
    factions: ArrayVec<[Faction; 2]>,
    rune_set: Box<str>,
}

#[derive(Deserialize, Debug)]
struct AbilitySet {
    abilities: ArrayVec<[FeedAbility; 4]>,
}

type Hash = [u8; 32];

pub async fn parse(previous: Option<Hash>) -> anyhow::Result<Option<(DB, Hash)>> {
    let start = Instant::now();
    // let feed = fs::File::open("./feed.json")?;
    // let reader = io::BufReader::new(feed);
    // let mut feed: Feed = serde_json::from_reader(reader)?;

    let mut hasher = Keccak::v256();
    // let feed = Parser::get("http://localhost:1337/feed.json", &mut hasher).await?;
    let feed = Parser::get("https://www.poxnora.com/api/feed.do?t=json", &mut hasher).await?;

    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);

    if let Some(previous) = previous {
        if previous == hash {
            log::info!("📜 Feed hash unchanged");
            return Ok(None);
        } else {
            log::info!("💡 Feed hash has changed!")
        }
    }

    let mut feed: Feed = serde_json::from_slice(&feed)?;

    log::info!("📜 Parsed feed.json in {:?}", start.elapsed());

    let start = Instant::now();

    // Sort all by id so we get a good insertion order
    feed.champs.sort_unstable_by_key(|rune| rune.core.id());
    feed.spells.sort_unstable_by_key(|rune| rune.core.id());
    feed.equips.sort_unstable_by_key(|rune| rune.core.id());
    feed.relics.sort_unstable_by_key(|rune| rune.core.id());

    let mut db = DB::default();

    for &damage in &["Acid","Disease","Electricity","Fire","Frost","Magical","Physical","Poison","Psychic","Sonic"] {
        let entry = db.effects.entry(damage);

        entry.name = damage.into();
        entry.desc = format!("{} damage type.", damage).into();
        entry.kind = EffectKind::Damage;
    }

    for fchamp in feed.champs {
        let champ = db.champs.entry(fchamp.core);

        for core in fchamp.starting_abilities {
            let ability = db.abilities.entry(core);

            ability.group = db.ability_groups
                .entry(&ability.core.name)
                .rank(ability)
                .tag(&*champ)
                .id();

            champ.starting_abilities.push(ability.id());
        }
        let [left, right] = fchamp.ability_sets;
        for fability in left.abilities {
            let ability = db.abilities.entry(fability.core);

            ability.group = db.ability_groups
                .entry(&ability.core.name)
                .rank(ability)
                .tag(&*champ)
                .id();

            champ.ability_sets[0].push(ability.id());
            if fability.default == Some(true) {
                champ.defaults[0] = ability.id();
            }
        }
        for fability in right.abilities {
            let ability = db.abilities.entry(fability.core);

            ability.group = db.ability_groups
                .entry(&ability.core.name)
                .rank(ability)
                .tag(&*champ)
                .id();

            champ.ability_sets[1].push(ability.id());
            if fability.default == Some(true) {
                champ.defaults[1] = ability.id();
            }
        }
        for class in fchamp.classes {
            champ.classes.push(
                db.classes.entry(&class).add(champ).id()
            );
        }
        for race in fchamp.races {
            champ.races.push(
                db.races.entry(&race).add(champ).id()
            );
        }
        for faction in fchamp.factions.into_iter().take(2) {
            champ.factions.push(faction);
            db.factions.entry(faction).tag(&*champ);
        }
        champ.expansion = db.expansions.entry(&fchamp.rune_set).tag(&*champ).id();
        champ.artist = db.artists.entry(&fchamp.artist).tag(&*champ).id();
    }

    for fspell in feed.spells {
        let spell = db.spells.entry(fspell.core);

        for faction in fspell.factions {
            spell.factions.push(faction);
            db.factions.entry(faction).tag(&*spell);
        }
        spell.expansion = db.expansions.entry(&fspell.rune_set).tag(&*spell).id();
        spell.artist = db.artists.entry(&fspell.artist).tag(&*spell).id();
    }

    for fequip in feed.equips {
        let equip = db.equips.entry(fequip.core);

        for faction in fequip.factions {
            equip.factions.push(faction);
            db.factions.entry(faction).tag(&*equip);
        }
        equip.expansion = db.expansions.entry(&fequip.rune_set).tag(&*equip).id();
        equip.artist = db.artists.entry(&fequip.artist).tag(&*equip).id();
    }

    for frelic in feed.relics {
        let relic = db.relics.entry(frelic.core);

        for faction in frelic.factions {
            relic.factions.push(faction);
            db.factions.entry(faction).tag(&*relic);
        }
        relic.expansion = db.expansions.entry(&frelic.rune_set).tag(&*relic).id();
        relic.artist = db.artists.entry(&frelic.artist).tag(&*relic).id();
    }

    log::info!("🚧 Populated DB in {:?}", start.elapsed());

    // -----------------------------------

    let start = Instant::now();

    let mut parser = Parser::new();
    let mut slot = 0;

    // Iterator would screw mutable borrows inside,
    // mut iterator would screw immutable borrows inside
    while let Some(ability) = db.abilities.at(slot) {
        let name = &ability.core.name;
        let desc = &ability.core.short_description;
        let tag_id = EntityId::AbilityGroup(ability.group);

        if let Some(fixed) = parser.fix_desc(&db, name, desc) {
            db.abilities.at_mut(slot).unwrap().core.short_description = fixed.into();

            parser.tag(&mut db, tag_id);
        }

        slot += 1;
    }

    slot = 0;

    while let Some(spell) = db.spells.at(slot) {
        let name = &spell.core.raw.name;
        let desc = &spell.core.raw.description;
        let tag_id = EntityId::Spell(spell.core.raw.id);

        if let Some(fixed) = parser.fix_desc(&db, name, desc) {
            db.spells.at_mut(slot).unwrap().core.raw.description = fixed.into();

            parser.tag(&mut db, tag_id);
        }

        slot += 1;
    }

    slot = 0;

    while let Some(equip) = db.equips.at(slot) {
        let name = &equip.core.raw.name;
        let desc = &equip.core.raw.description;
        let tag_id = EntityId::Equip(equip.core.raw.id);

        if let Some(fixed) = parser.fix_desc(&db, name, desc) {
            db.equips.at_mut(slot).unwrap().core.raw.description = fixed.into();

            parser.tag(&mut db, tag_id);
        }

        slot += 1;
    }

    slot = 0;

    while let Some(relic) = db.relics.at(slot) {
        let name = &relic.core.raw.name;
        let desc = &relic.core.raw.description;
        let tag_id = EntityId::Relic(relic.core.raw.id);

        if let Some(fixed) = parser.fix_desc(&db, name, desc) {
            db.relics.at_mut(slot).unwrap().core.raw.description = fixed.into();

            parser.tag(&mut db, tag_id);
        }

        slot += 1;
    }

    log::info!("⚔️  Cross-references abilities and conditions in {:?}", start.elapsed());

    // println!("{:#?}", db.effects);

    Ok(Some((db, hash)))
}

struct Parser {
    buffer: String,
    tags_re: Regex,
    value_re: Regex,
    tags: Vec<TagId>,
}


pub enum TagId {
    Effect(Id),
    AbilityGroup(Id),
}

impl Parser {
    fn new() -> Self {
        Parser {
            buffer: String::with_capacity(1024),
            tags_re: Regex::new(r"(?x)
                <(ability|condition|mechanic) ([^>]+)>
                    ([^<]+)
                </(ability|condition|mechanic)>
                |
                (Acid|Disease|Electricity|Fire|Frost|Magical|Physical|Poison|Psychic|Sonic)\s(?:damage|attack)
            ").unwrap(),
            value_re: Regex::new(r"\bvalue=([^ >]+)").unwrap(),
            tags: Vec::new(),
        }
    }

    async fn get<H>(url: &str, hasher: &mut H) -> anyhow::Result<Vec<u8>>
    where
        H: Hasher,
    {
        use futures_util::StreamExt;

        log::info!("⬇️  Downloading and hashing {}...", url);

        let request = reqwest::get(url).await?;
        let capacity = request.content_length().unwrap_or(1024) as usize;

        let mut stream = request.bytes_stream();
        let mut bytes = Vec::with_capacity(capacity);

        while let Some(item) = stream.next().await {
            let chunk = item?;

            hasher.update(&chunk);
            bytes.extend_from_slice(&chunk);
        }

        Ok(bytes)
    }

    fn fix_desc(&mut self, db: &DB, name: &str, desc: &str) -> Option<&str> {
        let mut last = 0;

        self.buffer.clear();
        self.tags.clear();

        for caps in self.tags_re.captures_iter(desc) {
            if let Some(m) = caps.get(5) {
                let damage = m.as_str();

                self.buffer.push_str(&desc[last..m.start()]);
                last = m.end();

                if let Some(effect) = db.effects.get_by_key(&Effect::make_key(damage)) {
                    self.tags.push(TagId::Effect(effect.id()));

                    write!(&mut self.buffer, "[{}](/effect/{})", damage, &effect.key).unwrap();
                } else {
                    log::warn!("⚠️  Unknown damage type <{}> inside <{}>", damage, name);
                    write!(&mut self.buffer, "[{}](*)", damage).unwrap();
                }

                continue;
            }

            let tag = &caps[1];
            let text = &caps[3];

            if tag != &caps[4] {
                log::warn!("⚠️  Incorrectly closed tag in {}", name);
            }

            let value = match self.value_re.captures(&caps[2]) {
                Some(c) => c.get(1).unwrap().as_str(),
                None => {
                    log::warn!("⚠️  Missing value for <{}> inside <{}>", text, name);
                    ""
                }
            };
            let (id, tag) = match tag {
                "ability" => {
                    let aid: Id = match value.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            log::warn!("⚠️  Invalid value <{}> for <{}> inside <{}>", value, text, name);
                            !0
                        }
                    };
                    let gid = match db.abilities.get(aid) {
                        Some(ability) => {
                            self.tags.push(TagId::AbilityGroup(ability.group));

                            ability.group
                        }
                        None => {
                            log::warn!("⚠️  Unknown ability <{}> {} inside <{}>", text, aid, name);
                            !0
                        }
                    };

                    (gid, "ability")
                },
                "condition" => (!0, "effect"),
                "mechanic" => (!0, "effect"),
                _ => unreachable!(),
            };

            let m = caps.get(0).unwrap();
            self.buffer.push_str(&desc[last..m.start()]);
            last = m.end();

            if id != !0 {
                write!(&mut self.buffer, "[{}](/{}/{})", text, tag, id).unwrap();
            } else {
                write!(&mut self.buffer, "[{}](*)", text).unwrap();
            }
        }

        if self.buffer.len() != 0 {
            self.buffer.push_str(&desc[last..]);

            Some(&self.buffer)
        } else {
            None
        }
    }

    fn tag(&mut self, db: &mut DB, tag_id: EntityId) {
        for uid in &self.tags {
            match *uid {
                TagId::Effect(id) => {
                    db.effects[id].tags.tag(tag_id);
                },
                TagId::AbilityGroup(id) => {
                    db.ability_groups[id].tags.tag(tag_id);
                },
            }
        }
    }
}

pub fn create_search_index(db: &mut DB) {
    let start = Instant::now();

    for champ in db.champs.iter() {
        db.search.insert(&champ.core.raw.name, EntityId::Champion(champ.id()));
    }
    for spell in db.spells.iter() {
        db.search.insert(&spell.core.raw.name, EntityId::Spell(spell.id()));
    }
    for equip in db.equips.iter() {
        db.search.insert(&equip.core.raw.name, EntityId::Equip(equip.id()));
    }
    for relic in db.relics.iter() {
        db.search.insert(&relic.core.raw.name, EntityId::Relic(relic.id()));
    }
    for ability in db.ability_groups.iter() {
        db.search.insert(&ability.name, EntityId::AbilityGroup(ability.id()));
    }
    for effect in db.effects.iter() {
        db.search.insert(&effect.name, EntityId::Effect(effect.id));
    }

    log::info!("🔎 Created search index in {:?} ({}kb)", start.elapsed(), db.search.size() / 1024);
    // println!("{:#?}", db.search);
}
