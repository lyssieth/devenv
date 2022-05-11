use std::{
    fs::{read_to_string, File},
    ops::Deref,
    path::PathBuf,
};

use cached::proc_macro::once;
use serde::{Deserialize, Serialize};

use crate::{root, Res};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub languages: Vec<Element>,
    pub platforms: Vec<Element>,
    pub tools: Vec<Tool>,
}

impl Configuration {
    pub fn path() -> PathBuf {
        let mut config = root();

        config.push("config.yml");

        config
    }

    pub fn exists() -> bool {
        Self::path().exists()
    }

    pub fn find_tool(&self, name: &str) -> Option<Tool> {
        self.tools
            .iter()
            .find(|tool| {
                tool.element.name == name || tool.element.aliases.iter().any(|alias| alias == name)
            })
            .cloned()
    }

    pub fn find_language(&self, name: &str) -> Option<Element> {
        if name == "any" {
            return Some(Element::default());
        }

        self.languages
            .iter()
            .find(|language| {
                language.name == name || language.aliases.iter().any(|alias| alias == name)
            })
            .cloned()
    }

    pub fn find_platform(&self, name: &str) -> Option<Element> {
        if name == "any" {
            return Some(Element::default());
        }

        self.platforms
            .iter()
            .find(|platform| {
                platform.name == name || platform.aliases.iter().any(|alias| alias == name)
            })
            .cloned()
    }

    pub fn save(&self) -> Res<()> {
        let path = Self::path();

        let mut file = File::create(&path)?;

        serde_yaml::to_writer(&mut file, &self)?;

        Ok(())
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            languages: vec![Element {
                name: "rust".to_string(),
                aliases: vec!["rs".to_string()],
            }],
            platforms: vec![Element {
                name: "x86".to_string(),
                aliases: vec!["x86_64".to_string(), "x64".to_string()],
            }],
            tools: vec![
                Tool {
                    element: Element {
                        name: "docker".to_string(),
                        aliases: vec!["dockerfile".to_string()],
                    },
                    filename: "Dockerfile".to_string(),
                },
                Tool {
                    element: Element {
                        name: "drone".to_string(),
                        aliases: vec!["drone.yml".to_string(), ".drone.yml".to_string()],
                    },
                    filename: ".drone.yml".to_string(),
                },
            ],
        }
    }
}

#[once(result = true)]
pub fn load() -> Res<Configuration> {
    let path = Configuration::path();

    let content = read_to_string(&path)?;

    Ok(serde_yaml::from_str(&content)?)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Element {
    pub name: String,
    pub aliases: Vec<String>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            name: "any".to_string(),
            aliases: vec![],
        }
    }
}

impl Deref for Element {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tool {
    #[serde(flatten)]
    pub element: Element,
    pub filename: String,
}

impl Deref for Tool {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}
