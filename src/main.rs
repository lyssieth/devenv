#![warn(clippy::pedantic)]

use std::{fmt, path::PathBuf, str::FromStr};

use argh::FromArgs;
use color_eyre::{eyre::eyre, Report};
use paris::warn;
use serde::{Deserialize, Serialize};

mod files;

mod create;
mod generate;

type Res<T> = Result<T, Report>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SupportedKinds {
    Docker,
    Drone,
    Just,
}

impl SupportedKinds {
    #[must_use]
    pub fn filename(&self) -> String {
        match self {
            Self::Docker => "Dockerfile".to_string(),
            Self::Drone => ".drone.yml".to_string(),
            Self::Just => "Justfile".to_string(),
        }
    }
}

impl fmt::Display for SupportedKinds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Docker => write!(f, "docker"),
            Self::Drone => write!(f, "drone"),
            Self::Just => write!(f, "just"),
        }
    }
}

impl FromStr for SupportedKinds {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "docker" | "dockerfile" => Ok(Self::Docker),
            "drone" => Ok(Self::Drone),
            "just" | "justfile" => Ok(Self::Just),

            _ => Err(eyre!("Unknown kind: {}", s)),
        }
    }
}

fn main() -> Res<()> {
    color_eyre::install()?;

    let args: Args = argh::from_env();

    match args.command {
        Command::Create(_) => create::run(args),
        Command::Generate(_) => generate::run(args),
    }?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Template {
    #[serde(rename = "ProjectName")]
    project_name: String,
}

/// An application for managing various development environment things.
///
/// The `language` and `platform` arguments are case-sensitive.
#[derive(Debug, FromArgs)]
struct Args {
    /// the platform we're on (e.g. `arm`, `x86`). default 'x86'
    #[argh(option, short = 'p', default = r#""x86".to_string()"#)]
    platform: String,

    /// the language we're using (e.g. `rust`, `python`). default 'rust'
    #[argh(option, short = 'l', default = r#""rust".to_string()"#)]
    language: String,

    #[argh(subcommand)]
    command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Command {
    Create(Create),
    Generate(Generate),
}

impl Command {
    pub fn create(self) -> Create {
        match self {
            Self::Create(c) => c,
            Self::Generate(_) => unreachable!("optimize me son"),
        }
    }

    pub fn generate(self) -> Generate {
        match self {
            Self::Create(_) => unreachable!("optimize me son"),
            Self::Generate(c) => c,
        }
    }
}

/// Stores a new type of file for future use
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "create")]
struct Create {
    /// the kind of file we're creating
    #[argh(positional)]
    kind: SupportedKinds,

    /// the file we're using as the template
    #[argh(positional)]
    template: PathBuf,
}

/// Generates a file based on the current environment
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "generate")]
struct Generate {
    /// the kind of file we're generating
    #[argh(positional)]
    kinds: Vec<SupportedKinds>,
}
