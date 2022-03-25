#![warn(clippy::pedantic)]

use std::{fmt, fs::File, io::Write, str::FromStr};

use clap::Parser;
use color_eyre::Report;
use paris::{error, warn};

mod files;

type Res<T> = Result<T, Report>;

fn main() -> Res<()> {
    color_eyre::install()?;

    let Args { file_types } = Args::parse();

    for x in file_types {
        match x {
            FileType::Dockerfile => generate_dockerfile(),
            FileType::Justfile => generate_justfile(),
        }?;
    }

    Ok(())
}

fn generate_dockerfile() -> Res<()> {
    let cwd = std::env::current_dir()?;

    let rd = cwd.read_dir()?;

    let mut project_language = ProjectLanguage::Unknown;

    for x in rd {
        let x = x?;

        let file_name = x.file_name();
        let file_name = file_name.to_string_lossy();
        let file_name = file_name.as_ref();

        if file_name == "Dockerfile" {
            warn!("Dockerfile already exists, will not overwrite.");
            return Ok(());
        }

        match file_name {
            "Cargo.toml" | "Cargo.lock" => {
                project_language = ProjectLanguage::Rust;
                break;
            }
            _ => {}
        };

        if file_name.ends_with("csproj") || file_name.ends_with("sln") {
            project_language = ProjectLanguage::Dotnet;
            break;
        }
    }

    let project_name = cwd.file_name().unwrap();
    let project_name = project_name.to_string_lossy();
    let project_name = project_name.as_ref();

    let mut base = match project_language {
        ProjectLanguage::Rust => files::DOCKERFILE_RUST.to_owned(),
        ProjectLanguage::Dotnet => files::DOCKERFILE_DOTNET.to_owned(),
        ProjectLanguage::Unknown => {
            error!("Could not determine project language.");
            return Ok(());
        }
    };

    base = replace_keyword(&base, Keyword::ProjectName, project_name);
    base = replace_keyword(
        &base,
        Keyword::ProjectNameLowercase,
        &project_name.to_lowercase(),
    );
    base = replace_keyword(
        &base,
        Keyword::ProjectNameDashesToUnderscores,
        &project_name.replace('-', "_"),
    );

    let mut file = File::create("Dockerfile")?;

    file.write_all(base.as_bytes())?;

    Ok(())
}

fn generate_justfile() -> Res<()> {
    let cwd = std::env::current_dir()?;

    let project_name = cwd.file_name().unwrap();
    let project_name = project_name.to_string_lossy();
    let project_name = project_name.as_ref();

    let mut base = files::JUSTFILE.to_owned();

    base = replace_keyword(&base, Keyword::ProjectName, project_name);
    base = replace_keyword(
        &base,
        Keyword::ProjectNameLowercase,
        &project_name.to_lowercase(),
    );
    base = replace_keyword(
        &base,
        Keyword::ProjectNameDashesToUnderscores,
        &project_name.replace('-', "_"),
    );

    let mut file = File::create("Justfile")?;

    file.write_all(base.as_bytes())?;

    Ok(())
}

fn replace_keyword(inside: &str, keyword: Keyword, with: &str) -> String {
    let mut result = inside.to_owned();

    let keyword = keyword.to_string();

    result = result.replace(&keyword, with);

    result
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum Keyword {
    ProjectName,
    ProjectNameDashesToUnderscores,
    ProjectNameLowercase,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::ProjectName => write!(f, "{{ProjectName}}"),
            Keyword::ProjectNameDashesToUnderscores => {
                write!(f, "{{ProjectName_DashesToUnderscores}}")
            }
            Keyword::ProjectNameLowercase => write!(f, "{{ProjectName_Lowercase}}"),
        }
    }
}

/// A program to automatically generate development environment files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Type of file to generate
    file_types: Vec<FileType>,
}

#[derive(Debug)]
enum ProjectLanguage {
    Rust,
    Dotnet,
    Unknown,
}

#[derive(Debug)]
enum FileType {
    Dockerfile,
    Justfile,
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dockerfile" | "docker" => Ok(FileType::Dockerfile),
            "justfile" | "just" => Ok(FileType::Justfile),
            _ => Err(format!("Unknown file type: {}", s)),
        }
    }
}
