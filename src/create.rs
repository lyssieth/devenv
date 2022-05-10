use std::fs;

use crate::{
    files::{self, DevFile},
    Args, Create, Res,
};

pub(super) fn run(args: Args) -> Res<()> {
    let Args {
        platform,
        language,
        command,
    } = args;
    let create = command.create();
    let Create { kind, template } = create;

    let text = fs::read_to_string(&template)?;

    let df = DevFile {
        language,
        platform,
        which: kind,
        data: text,
    };

    files::create_file(&df)?;

    Ok(())
}
