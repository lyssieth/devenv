use std::{fs::File, io::Write, process};

use paris::error;
use tinytemplate::TinyTemplate;

use crate::{
    config::{self, Configuration, Element, Tool},
    files, Args, Command, Generate, Res, Template,
};

#[derive(Debug)]
pub struct Arguments {
    pub tools: Vec<Tool>,
    pub platform: Element,
    pub language: Element,
}

impl From<Args> for Arguments {
    fn from(args: Args) -> Self {
        let Args {
            command,
            language,
            platform,
        } = args;

        let kinds = match command {
            Command::Generate(Generate { kinds }) => kinds,

            _ => {
                unreachable!("we shouldn't be able to call this when it's generate")
            }
        };

        let cfg = config::load().expect("Failed to load configuration file.");

        let tools = kinds
            .into_iter()
            .filter_map(|kind| {
                let tool = cfg.find_tool(&kind);

                if let Some(tool) = tool {
                    Some(tool)
                } else {
                    error!("Unknown tool: {}", kind);
                    error!("Please go define it in {:?}", Configuration::path());
                    None
                }
            })
            .collect();

        let platform = cfg.find_platform(&platform).unwrap_or_else(|| {
            error!("Unknown platform: {}", platform);
            error!("Please go define it in {:?}", Configuration::path());
            process::exit(1);
        });

        let language = cfg.find_language(&language).unwrap_or_else(|| {
            error!("Unknown language: {}", language);
            error!("Please go define it in {:?}", Configuration::path());
            process::exit(1);
        });

        Self {
            tools,
            platform,
            language,
        }
    }
}

pub(super) fn run(args: Args) -> Res<()> {
    let Arguments {
        tools,
        platform,
        language,
    } = args.into();

    for tool in tools {
        let f = files::get_file(&tool, &platform, &language);

        let file = match f {
            Ok(f) => f,
            Err(e) => {
                error!(
                    "There is no matching template file {platform}-{language} for {tool}: {e}",
                    platform = platform.as_str(),
                    language = language.as_str(),
                    tool = tool.as_str()
                );
                continue;
            }
        };

        let data = file.data;

        let mut tt = TinyTemplate::new();

        tt.add_template("data", &data)?;

        let output = tt.render("data", &Template::from_project_name(&get_project_name()))?;

        File::create(&tool.filename)?.write_all(output.as_bytes())?;
    }

    Ok(())
}

fn get_project_name() -> String {
    let pwd = std::env::current_dir().unwrap();

    pwd.file_name().unwrap().to_str().unwrap().to_string()
}
