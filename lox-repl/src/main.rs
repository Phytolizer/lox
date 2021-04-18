use std::fs::File;
use std::io;
use std::io::stdin;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

use dialoguer::console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use io::Read;
use log::error;
use scanner::Scanner;
use structopt::StructOpt;

mod object;
mod scanner;
mod token;
mod token_type;

static mut HAD_ERROR: bool = false;

#[derive(StructOpt)]
struct Opt {
    script: Option<PathBuf>,
}

fn main() {
    pretty_env_logger::init();
    let opt = Opt::from_args();

    match opt.script {
        Some(script) => run_file(script).unwrap_or_else(|e| {
            error!("{}", e);
            process::exit(1)
        }),
        None => run_prompt().unwrap_or_else(|e| {
            error!("{}", e);
            process::exit(1)
        }),
    }
}

fn run_file(script: PathBuf) -> io::Result<()> {
    let mut bytes = vec![];
    File::open(script)?.read_to_end(&mut bytes)?;
    run(String::from_utf8(bytes).unwrap_or_else(|e| {
        error!("{}", e);
        process::exit(1)
    }));
    if unsafe { HAD_ERROR } {
        process::exit(65);
    }
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let theme = ColorfulTheme {
        prompt_prefix: style("> ".into()),
        ..Default::default()
    };
    let mut input = Input::with_theme(&theme);
    loop {
        let line: String = input.interact().unwrap_or_else(|e| {
            error!("{}", e);
            process::exit(1)
        });

        run(line);

        unsafe { HAD_ERROR = false };
    }
}

fn run(source: String) {
    let scanner = Scanner::new(&source);

    for token in scanner.scan_tokens() {
        println!("{}", token);
    }
}

fn err(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, loc: &str, message: &str) {
    error!("[line {}] Error{}: {}", line, loc, message);
    unsafe { HAD_ERROR = true };
}
