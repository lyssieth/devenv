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

use bincode::{
    config,
    serde::{decode_from_std_read, encode_into_std_write},
};
use cached::proc_macro::cached;
use color_eyre::eyre::eyre;
use paris::warn;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Element, Tool},
    root, Res,
};

#[cached(key = "String", convert = r#"{ which.name.clone() }"#)]
fn which(which: &Tool) -> PathBuf {
    let mut root = root();

    root.push(&which.name);

    root
}

/// We use an extension so it's obvious that it's not _just_ an editable file.
const EXTENSION: &str = "bc";

pub fn get_file(tool: &Tool, platform: &Element, language: &Element) -> Res<DevFile> {
    let path = which(tool).join(format!("{}-{}.{EXTENSION}", platform.name, language.name));

    if !which(tool).exists() {
        fs::create_dir(which(tool))?;
    }

    let path = if path.exists() {
        path
    } else {
        which(tool).join(format!("{}-{}.{EXTENSION}", platform.name, "any"))
    };

    if !path.exists() {
        warn!(
            "There is no `{language}` or `any` file for `{platform}` & `{tool}`",
            language = language.name,
            platform = platform.name,
            tool = tool.name
        );
        return Err(eyre!("No file found"));
    }

    DevFile::load(&path)
}

pub fn create_file(df: &DevFile) -> Res<()> {
    let path = which(&df.tool).join(format!(
        "{}-{}.{EXTENSION}",
        &df.platform.name, &df.language.name
    ));

    if !which(&df.tool).exists() {
        fs::create_dir(which(&df.tool))?;
    }

    df.save(&path)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DevFile {
    pub language: Element,
    pub platform: Element,
    pub tool: Tool,
    pub data: String,
}

impl DevFile {
    pub fn save(&self, path: &Path) -> Res<()> {
        let mut file = std::fs::File::create(path)?;

        encode_into_std_write(self, &mut file, config::standard())?;

        Ok(())
    }

    pub fn load(path: &Path) -> Res<Self> {
        let mut file = std::fs::File::open(path)?;

        let res = decode_from_std_read(&mut file, config::standard())?;

        Ok(res)
    }
}
