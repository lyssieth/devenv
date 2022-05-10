use std::{fs::File, io::Write};

use paris::error;
use tinytemplate::TinyTemplate;

use crate::{files, Args, Generate, Res, Template};

pub(super) fn run(args: Args) -> Res<()> {
    let Args {
        platform,
        language,
        command,
    } = args;
    let generate = command.generate();

    let Generate { kinds } = generate;

    for x in kinds {
        let f = files::get_file(x, &platform, &language);

        let file = match f {
            Ok(f) => f,
            Err(e) => {
                error!("There is no matching dev-file {platform}-{language} for {x}: {e}");
                continue;
            }
        };

        let data = file.data;

        let mut tt = TinyTemplate::new();

        tt.add_template("data", &data)?;

        let output = tt.render(
            "data",
            &Template {
                project_name: get_project_name(),
            },
        )?;

        File::create(&x.filename())?.write_all(output.as_bytes())?;
    }

    Ok(())
}

fn get_project_name() -> String {
    let pwd = std::env::current_dir().unwrap();

    pwd.file_name().unwrap().to_str().unwrap().to_string()
}
