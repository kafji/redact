use ::redact::*;
use anyhow::{Context, Result};
use std::{
    env, fs,
    io::{self, Read, Write},
};

struct Args {
    input: String,
    file_mode: bool,
}

fn main() -> Result<()> {
    let mut app = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("input")
                .required(true)
                .value_name("TEXT")
                .help("A literal text to redact"),
        )
        .arg(
            clap::Arg::with_name("file_mode")
                .short("f")
                .long("file")
                .help("Takes <TEXT> as a file path instead of a literal text"),
        );

    let args = match app.get_matches_from_safe_borrow(env::args_os()) {
        Ok(matches) => {
            let input = (*matches.value_of_lossy("input").unwrap()).to_owned();
            let file_mode = matches.is_present("file_mode");
            Args { input, file_mode }
        }
        Err(error) => match error.kind {
            clap::ErrorKind::MissingRequiredArgument => {
                if atty::is(atty::Stream::Stdin) {
                    let stdout = io::stdout();
                    let mut stdout = stdout.lock();
                    app.write_help(&mut stdout)
                        .with_context(|| "Failed to write output to stdout.")?;
                    stdout
                        .write_all(b"\n\n")
                        .with_context(|| "Failed to write output to stdout.")?;
                    return Ok(());
                }
                let mut input = String::new();
                io::stdin()
                    .read_to_string(&mut input)
                    .with_context(|| "Failed to read input from stdin.")?;
                let file_mode = false;
                Args { input, file_mode }
            }
            _ => return Err(anyhow::Error::from(error)),
        },
    };

    let config = Config::from_env_var().with_context(|| "Failed to get configuration.")?;

    let text = if args.file_mode {
        let path = &args.input;
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read input file with path: {}", path))?
    } else {
        args.input
    };

    let output = redact(config.keywords(), &text).with_context(|| "Failed to redact.")?;

    println!("{}", &output);

    Ok(())
}
