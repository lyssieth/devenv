#![warn(clippy::pedantic)]

use std::{
    fmt::Debug,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use argh::FromArgs;
use color_eyre::{eyre, Report};
use paris::{info, warn};
use serde::{Deserialize, Serialize};

mod config;
mod create;
mod files;
mod generate;

type Res<T> = Result<T, Report>;

static ROOT_CHECK: AtomicBool = AtomicBool::new(false);

fn root() -> PathBuf {
    let mut config = dirs::config_dir().unwrap();

    config.push("devenv");

    if !ROOT_CHECK.load(Ordering::SeqCst) && !config.exists() {
        fs::create_dir_all(&config).unwrap_or_else(|_| panic!("Failed to create {:?}", &config));

        ROOT_CHECK.swap(true, Ordering::SeqCst);
    }

    config
}

fn main() -> Res<()> {
    color_eyre::install()?;

    if !config::Configuration::exists() {
        config::Configuration::default().save()?;

        info!(
            "Created a default configuration at {:?}. We suggest editing it before further use.",
            config::Configuration::path()
        );
    }

    let args: Args = argh::from_env();

    match args.command {
        Command::Create(_) => create::run(args),
        Command::Generate(_) => generate::run(args),
        Command::Config(cfg) => cfg.action.run(),
    }?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Template {
    #[serde(rename = "ProjectName")]
    project_name: String,

    #[serde(rename = "ProjectName_DashesToUnderscores")]
    project_name_dashes_to_underscores: String,

    #[serde(rename = "ProjectName_Lowercase")]
    project_name_lowercase: String,
}

impl Template {
    pub fn from_project_name(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
            project_name_dashes_to_underscores: project_name.replace('-', "_"),
            project_name_lowercase: project_name.to_lowercase(),
        }
    }
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
    Config(Config),
}

/// Stores a new type of file for future use
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "create")]
struct Create {
    /// the tool we're creating a file for
    #[argh(positional)]
    tool: String,

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
    kinds: Vec<String>,
}

/// Actions relating to the config
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "config")]
struct Config {
    /// the action to do. `regenerate` regenerates the config from the in-built default, `show` shows the current config, and `path` shows the path.
    #[argh(positional)]
    action: Action,
}

#[derive(Debug)]
enum Action {
    Regenerate,
    Show,
    Path,
}

impl Action {
    pub fn run(&self) -> Res<()> {
        match self {
            Self::Regenerate => config::Configuration::default().save(),
            Self::Show => {
                println!("{:#?}", config::load()?);
                Ok(())
            }
            Self::Path => {
                println!("{:?}", config::Configuration::path());
                Ok(())
            }
        }
    }
}

impl FromStr for Action {
    type Err = Report;

    fn from_str(s: &str) -> Res<Self> {
        match s.to_lowercase().as_str() {
            "regenerate" => Ok(Self::Regenerate),
            "show" => Ok(Self::Show),
            "path" => Ok(Self::Path),

            _ => Err(eyre::eyre!("Unknown action: {}", s)),
        }
    }
}
