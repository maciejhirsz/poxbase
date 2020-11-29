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

use std::fs;
use std::path::{PathBuf, Path};
use futures::join;

use crate::db::DB;

const CDN: &str = "https://d2aao99y1mip6n.cloudfront.net";
const ASSETS: &str = "../frontend/public/assets";

#[derive(Clone, Copy)]
enum ArtStatus {
    NoChange = 0,
    Success = 1,
    Fail = 3,
}

impl std::ops::BitOr for ArtStatus {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        let bits = (self as u32) | (rhs as u32);

        match bits {
            0 => ArtStatus::NoChange,
            1 => ArtStatus::Success,
            _ => ArtStatus::Fail,
        }
    }
}

impl ArtStatus {
    fn log(self, category: &str, name: &str) {
        match self {
            ArtStatus::NoChange => (),
            ArtStatus::Success => {
                log::info!("🎨 Downloaded art for [{}] {}", category, name);
            }
            ArtStatus::Fail => {
                log::warn!("💔 Missing art for [{}] {}", category, name);
            }
        }
    }
}

async fn download(url: &str, path: &Path) -> ArtStatus {
    async fn work(url: &str, path: &Path) -> anyhow::Result<()> {
        use futures_util::StreamExt;
        use std::io::{BufWriter, Write};

        let res = reqwest::get(url).await?;

        if !res.status().is_success() {
            return Err(crate::error::NotFound.into());
        }
        let mut stream = reqwest::get(url).await?.bytes_stream();
        let mut file = BufWriter::new(fs::File::create(path)?);

        while let Some(item) = stream.next().await {
            file.write_all(&item?)?;
        }

        Ok(())
    };

    match work(url, path).await {
        Ok(()) => {
            log::info!("⬇️  {}", url);
            ArtStatus::Success
        }
        Err(err) => {
            log::error!("❌ {} {:?}", url, err);
            ArtStatus::Fail
        }
    }
}

pub async fn check(db: &mut DB) {
    async fn check_rune(hash: &str) -> ArtStatus {
        let rune = PathBuf::from(format!("{}/runes/{}.jpg", ASSETS, hash));

        if !rune.exists() {
            let url = format!("{}/images/runes/lg/{}.jpg", CDN, hash);
            download(&url, &rune).await
        } else {
            ArtStatus::NoChange
        }
    }

    async fn check_mini(hash: &str) -> ArtStatus {
        let mini = PathBuf::from(format!("{}/minis/{}.png", ASSETS, hash));

        if !mini.exists() {
            let url = format!("{}/images/runes/sm/{}.png", CDN, hash);
            download(&url, &mini).await
        } else {
            ArtStatus::NoChange
        }
    }

    async fn check_idol(hash: &str) -> ArtStatus {
        let idol = PathBuf::from(format!("{}/idols/{}.gif", ASSETS, hash));

        if !idol.exists() {
            let url = format!("{}/images/runes/idols/{}.gif", CDN, hash);
            download(&url, &idol).await
        } else {
            ArtStatus::NoChange
        }
    }

    for champ in db.champs.iter() {
        let hash = &champ.core.raw.hash;
        let (r, m, i) = join!(check_rune(hash), check_mini(hash), check_idol(hash));

        (r | m | i).log("champ", &champ.core.raw.name);
    }

    for spell in db.spells.iter() {
        let hash = &spell.core.raw.hash;
        let (r, m) = join!(check_rune(hash), check_mini(hash));

        (r | m).log("spell", &spell.core.raw.name);
    }

    for equip in db.equips.iter() {
        let hash = &equip.core.raw.hash;
        let (r, m) = join!(check_rune(hash), check_mini(hash));

        (r | m).log("equip", &equip.core.raw.name);
    }

    for relic in db.relics.iter() {
        let hash = &relic.core.raw.hash;
        let (r, m, i) = join!(check_rune(hash), check_mini(hash), check_idol(hash));

        (r | m | i).log("relic", &relic.core.raw.name);
    }

    for ability in db.abilities.iter() {
        let icon = &*ability.core.icon_name;
        let path = PathBuf::from(format!("{}/big_icons/icon_{}.png", ASSETS, ability.core.icon_name));

        if !path.exists() {
            log::warn!("{} has a missing icon ({})", &ability.core.name, &ability.core.icon_name);
            // let url = format!("{}/images/ability_icons/small/icon_{}.gif", CDN, icon);

            // download(&url, &path).await.log("ability", &ability.core.name);
        }
    }
}
