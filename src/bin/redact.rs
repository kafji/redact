use ::redact::*;
use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("input")
                .required(true)
                .value_name("TEXT")
                .help("A literal text to redact"),
        )
        .arg(
            clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Takes <TEXT> as a file path instead of a literal text"),
        )
        .get_matches();

    let config = Config::from_env_var().with_context(|| "Failed to get configuration.")?;

    let result = if matches.is_present("file") {
        let path = matches.value_of_lossy("input").unwrap();
        let text = fs::read_to_string(&*path)
            .with_context(|| format!("Failed to read input file with path: {}", path))?;
        redact(config.keywords(), &text)
    } else {
        let text = matches.value_of_lossy("input").unwrap();
        redact(config.keywords(), &text)
    };
    let output = result.with_context(|| "Failed to redact.")?;

    println!("{}", &output);

    Ok(())
}
