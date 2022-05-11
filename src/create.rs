use std::{fs::read_to_string, process};

use paris::error;

use crate::{
    config::{self, Configuration, Element, Tool},
    files::{self, DevFile},
    Args, Command, Create, Res,
};

#[derive(Debug)]
pub struct Arguments {
    pub tool: Tool,
    pub platform: Element,
    pub language: Element,
    pub data: String,
}

impl From<Args> for Arguments {
    fn from(args: Args) -> Self {
        let Args {
            command,
            language,
            platform,
        } = args;

        let (tool, template) = match command {
            Command::Create(Create { tool, template }) => (tool, template),

            _ => {
                unreachable!("we shouldn't be able to call this when it's generate")
            }
        };

        let cfg = config::load().expect("Failed to load configuration file.");

        let tool = cfg
            .find_tool(&tool)
            .unwrap_or_else(|| {
                error!("Unknown tool: {}", tool);
                error!("Please go define it in {:?}", Configuration::path());
                process::exit(1);
            })
            .clone();

        let platform = cfg
            .find_platform(&platform)
            .unwrap_or_else(|| {
                error!("Unknown platform: {}", platform);
                error!("Please go define it in {:?}", Configuration::path());
                process::exit(1);
            })
            .clone();

        let language = cfg
            .find_language(&language)
            .unwrap_or_else(|| {
                error!("Unknown language: {}", language);
                error!("Please go define it in {:?}", Configuration::path());
                process::exit(1);
            })
            .clone();

        Self {
            tool,
            platform,
            language,
            data: read_to_string(&template).expect("Failed to read file"),
        }
    }
}

pub(super) fn run(args: Args) -> Res<()> {
    let Arguments {
        tool,
        platform,
        language,
        data,
    } = args.into();

    let df = DevFile {
        language,
        platform,
        tool,
        data,
    };

    files::create_file(&df)?;

    Ok(())
}
