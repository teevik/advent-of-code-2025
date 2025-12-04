#![feature(int_roundings)]
#![feature(cmp_minmax)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

use std::path::PathBuf;

use color_eyre::{Result, eyre::eyre};
use http_cache_surf::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};

pub fn fetch_input(day: u8) -> Result<String> {
    let session = std::env::var("SESSION").expect("should have a session token set");

    let client = surf::client().with(Cache(HttpCache {
        mode: CacheMode::ForceCache,
        manager: CACacheManager::new(PathBuf::from("cache"), false),
        options: HttpCacheOptions::default(),
    }));

    let url = format!("https://adventofcode.com/2025/day/{day}/input");

    let mut response =
        smol::block_on(client.send(surf::get(url).header("COOKIE", format!("session={session}"))))
            .map_err(|e| eyre!("{e}"))?;

    let text = smol::block_on(response.body_string()).map_err(|e| eyre!("{e}"))?;

    Ok(text)
}
