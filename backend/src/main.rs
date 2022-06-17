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

use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use actix_web::web::{Data, Path};
use actix_web::{
    dev, get, middleware, rt::time, rt::System, web, App, Error, HttpResponse, HttpServer,
};
use clap::Parser;
use futures::{
    future::{select, Either},
    pin_mut,
};
use serde::Serialize;
use simple_logger::SimpleLogger;

mod assets;
mod db;
mod error;
mod parser;
mod types;

use crate::db::{SearchId, Searchable, DB};
use crate::error::NotFound;
use crate::types::{
    Ability, AbilityGroup, Champion, Effect, EntityId, Equip, Id, Rarity, Relic, Rune, Shim, Spell,
};

#[derive(Parser)]
#[clap(
    name = "PoxBase",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "\
        PoxBase backend server  Copyright (C) 2020  Maciej Hirsz\n\n\
        This program comes with ABSOLUTELY NO WARRANTY;\n\
        This is free software, and you are welcome to redistribute it\n\
        under certain conditions; see LICENSE for details.",
)]
struct Opts {
    /// This is the socket the server is listening on. This is restricted localhost (127.0.0.1) by default and should
    /// be fine for most use cases. In a container, you likely want to set this to '0.0.0.0:8000'.
    #[clap(long = "listen", default_value = "127.0.0.1:8000")]
    socket: SocketAddr,
    /// Don't verify whether all assets have been downloaded
    #[clap(long = "no-assets")]
    no_assets: bool,
}

fn json<S: Serialize>(ser: &S) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::ser::to_string(ser)?))
}

#[get("/init")]
async fn get_init(db: Data<DB>) -> Result<HttpResponse, Error> {
    #[derive(Serialize)]
    struct Response<'a> {
        expansions: &'a [Shim<'a>],
    }

    let expansions = db
        .expansions
        .iter()
        .map(|xpack| xpack.shim())
        .collect::<Vec<_>>();

    json(&Response {
        expansions: &expansions,
    })
}

#[get("/typeahead/{query}")]
async fn get_typeahead(query: Path<String>, db: Data<DB>) -> Result<HttpResponse, Error> {
    #[derive(Serialize)]
    struct Response<'a> {
        results: Vec<Result<'a>>,
    }

    #[derive(Serialize)]
    struct Result<'a> {
        name: &'a str,
        #[serde(flatten)]
        sid: SearchId<'a>,
        rarity: Option<Rarity>,
    }

    let db = &*db;

    let response = Response {
        results: db
            .search
            .find(&query)
            .into_iter()
            .take(10)
            .map(|(eid, _)| {
                let (sid, name, rarity) = match eid {
                    EntityId::Champion(id) => {
                        let rune = db.champs.get(id).unwrap();

                        (
                            SearchId::Champion(id),
                            &rune.core.raw.name,
                            Some(rune.core.raw.rarity),
                        )
                    }
                    EntityId::Spell(id) => {
                        let rune = db.spells.get(id).unwrap();

                        (
                            SearchId::Spell(id),
                            &rune.core.raw.name,
                            Some(rune.core.raw.rarity),
                        )
                    }
                    EntityId::Equip(id) => {
                        let rune = db.equips.get(id).unwrap();

                        (
                            SearchId::Equip(id),
                            &rune.core.raw.name,
                            Some(rune.core.raw.rarity),
                        )
                    }
                    EntityId::Relic(id) => {
                        let rune = db.relics.get(id).unwrap();

                        (
                            SearchId::Relic(id),
                            &rune.core.raw.name,
                            Some(rune.core.raw.rarity),
                        )
                    }
                    EntityId::AbilityGroup(id) => (
                        SearchId::AbilityGroup(id),
                        &db.ability_groups[id].name,
                        None,
                    ),
                    EntityId::Effect(id) => {
                        let effect = &db.effects[id];
                        let sid = effect.search_id();

                        (sid, &effect.name, None)
                    }
                };

                Result { name, sid, rarity }
            })
            .collect(),
    };

    json(&response)
}

#[get("/champ/{id}")]
async fn get_champ(id: Path<Id>, db: Data<DB>) -> Result<HttpResponse, Error> {
    let db = &*db;
    let champ = db.champs.get(*id).ok_or(NotFound)?;

    #[derive(Serialize)]
    struct Response<'a> {
        champs: [&'a Champion; 1],
        abilities: Vec<&'a Ability>,
        classes: Vec<Shim<'a>>,
        races: Vec<Shim<'a>>,
        artists: [Shim<'a>; 1],
    }

    let abilities = champ
        .starting_abilities
        .iter()
        .chain(champ.ability_sets[0].iter())
        .chain(champ.ability_sets[1].iter())
        .map(|&id| db.abilities.get(id))
        .collect::<Option<_>>()
        .ok_or(NotFound)?;

    let classes = champ
        .classes
        .iter()
        .map(|&id| db.classes.get(id).map(|class| class.shim()))
        .collect::<Option<_>>()
        .ok_or(NotFound)?;

    let races = champ
        .races
        .iter()
        .map(|&id| db.races.get(id).map(|race| race.shim()))
        .collect::<Option<_>>()
        .ok_or(NotFound)?;

    let artists = [db.artists.get(champ.artist).ok_or(NotFound)?.shim()];

    json(&Response {
        champs: [champ],
        abilities,
        classes,
        races,
        artists,
    })
}

#[get("/spell/{id}")]
async fn get_spell(id: Path<Id>, db: Data<DB>) -> Result<HttpResponse, Error> {
    let db = &*db;
    let spell = db.spells.get(*id).ok_or(NotFound)?;
    let artists = [db.artists.get(spell.artist).ok_or(NotFound)?.shim()];

    #[derive(Serialize)]
    struct Response<'a> {
        spells: [&'a Rune<Spell>; 1],
        artists: [Shim<'a>; 1],
    }

    json(&Response {
        spells: [spell],
        artists,
    })
}

#[get("/equip/{id}")]
async fn get_equip(id: Path<Id>, db: Data<DB>) -> Result<HttpResponse, Error> {
    let db = &*db;
    let equip = db.equips.get(*id).ok_or(NotFound)?;
    let artists = [db.artists.get(equip.artist).ok_or(NotFound)?.shim()];

    #[derive(Serialize)]
    struct Response<'a> {
        equips: [&'a Rune<Equip>; 1],
        artists: [Shim<'a>; 1],
    }

    json(&Response {
        equips: [equip],
        artists,
    })
}

#[get("/relic/{id}")]
async fn get_relic(path: Path<Id>, db: Data<DB>) -> Result<HttpResponse, Error> {
    let db = &*db;
    let id = path.into_inner();
    let relic = db.relics.get(id).ok_or(NotFound)?;
    let artists = [db.artists.get(relic.artist).ok_or(NotFound)?.shim()];

    #[derive(Serialize)]
    struct Response<'a> {
        relics: [&'a Rune<Relic>; 1],
        artists: [Shim<'a>; 1],
    }

    json(&Response {
        relics: [relic],
        artists,
    })
}

#[get("/ability/{id}")]
async fn get_ability(id: Path<Id>, db: Data<DB>) -> Result<HttpResponse, Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response<'a> {
        ability_groups: [&'a AbilityGroup; 1],
        abilities: Vec<&'a Ability>,
    }

    let db = &*db;
    let group = db.ability_groups.get(*id).ok_or(NotFound)?;

    let abilities = group
        .ranks
        .iter()
        .map(|&id| db.abilities.get(id))
        .collect::<Option<_>>()
        .ok_or(NotFound)?;

    json(&Response {
        ability_groups: [group],
        abilities,
    })
}

#[get("/effect/{key}")]
async fn get_effect(key: Path<String>, db: Data<DB>) -> Result<HttpResponse, Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response<'a> {
        effects: [&'a Effect; 1],
    }

    let db = &*db;
    let effect = db.effects.get_by_key(&key).ok_or(NotFound)?;

    json(&Response { effects: [effect] })
}

struct BackgroundServer {
    server: dev::Server,
    system: System,
}

impl BackgroundServer {
    pub async fn stop(self) {
        self.server.stop(true).await;

        log::info!("⚙️  Server shut down, stopping system...");

        self.system.stop();
    }
}

fn spawn_server(db: DB, socket: SocketAddr) -> BackgroundServer {
    let db = Data::new(db);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || -> std::io::Result<()> {
        log::info!("⚙️  Starting a new server thread");

        let sys = System::new("poxbase-server");

        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
                .app_data(db.clone()) // Data<DB> is internally an Arc, so all this does is increment RC
                .service(get_init)
                .service(get_typeahead)
                .service(get_champ)
                .service(get_spell)
                .service(get_equip)
                .service(get_relic)
                .service(get_ability)
                .service(get_effect)
        })
        .bind(socket)?
        .shutdown_timeout(1)
        .run();

        let _ = tx.send(BackgroundServer {
            server,
            system: System::current(),
        });

        sys.run()?;

        log::info!("⚙️  Exited stopped server thread");

        Ok(())
    });

    rx.recv().unwrap()
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("Must be able to start a logger");

    let opts: Opts = Opts::parse();
    let interval = Duration::from_secs(3600 * 6);

    let mut prev_hash = None;
    let mut server: Option<BackgroundServer> = None;
    let mut interval = time::interval(interval);

    loop {
        let sigint = actix_web::rt::signal::ctrl_c();
        let tick = interval.tick();

        pin_mut!(sigint);
        pin_mut!(tick);

        if let Either::Left((sig, _)) = select(sigint, tick).await {
            sig.expect("Failed to read signal");

            log::info!("SIGINT received, exiting");
            break;
        }

        match parser::parse(prev_hash).await {
            Ok(Some((mut db, hash))) => {
                prev_hash = Some(hash);

                parser::create_search_index(&mut db);

                if !opts.no_assets {
                    assets::check(&mut db).await;
                }

                if let Some(server) = server.take() {
                    server.stop().await;
                }

                server = Some(spawn_server(db, opts.socket));
            }
            Ok(None) => (),
            Err(err) => {
                log::error!("❌ Failed parsing: {}", err);
            }
        }
    }

    Ok(())
}
