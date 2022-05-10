//! The general idea is that the directory structure is as follows:
//!
//! ```
//! devenv/
//!      - {which}/
//!               - {platform}-{language}.bc
//! ```
//!

use std::{
    fs,
    path::{Path, PathBuf},
};

use bincode::{DefaultOptions, Options};
use color_eyre::eyre::eyre;
use paris::warn;
use serde::{Deserialize, Serialize};

use crate::{Res, SupportedKinds};

fn root() -> PathBuf {
    let mut config = dirs::config_dir().unwrap();

    config.push("devenv");

    config
}

fn which(which: SupportedKinds) -> PathBuf {
    let mut root = root();

    root.push(which.to_string());

    root
}

/// We use an extension so it's obvious that it's not _just_ an editable file.
const EXTENSION: &str = ".bc";

pub fn get_file(sk: SupportedKinds, platform: &str, language: &str) -> Res<DevFile> {
    if !root().exists() {
        fs::create_dir_all(root())?;
    }
    let path = which(sk).join(format!("{}-{}.{}", platform, language, EXTENSION));

    if !which(sk).exists() {
        fs::create_dir(which(sk))?;
    }

    if !path.exists() {
        warn!("There is no `{language}` file for `{platform}` for `{sk}`");
        return Err(eyre!("No file found"));
    }

    DevFile::load(&path)
}

pub fn create_file(df: &DevFile) -> Res<()> {
    if !root().exists() {
        fs::create_dir_all(root())?;
    }

    let path = which(df.which).join(format!("{}-{}.{}", &df.platform, &df.language, EXTENSION));

    if !which(df.which).exists() {
        fs::create_dir(which(df.which))?;
    }

    df.save(&path)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DevFile {
    pub language: String,
    pub platform: String,
    pub which: SupportedKinds,
    pub data: String,
}

impl DevFile {
    pub fn save(&self, path: &Path) -> Res<()> {
        let mut file = std::fs::File::create(path)?;

        DefaultOptions::new()
            .allow_trailing_bytes()
            .serialize_into(&mut file, self)?;

        Ok(())
    }

    pub fn load(path: &Path) -> Res<Self> {
        let file = std::fs::File::open(path)?;

        let res = DefaultOptions::new()
            .allow_trailing_bytes()
            .deserialize_from(file)?;

        Ok(res)
    }
}
