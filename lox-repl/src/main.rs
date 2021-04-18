use std::fs::File;
use std::io;
use std::io::stdin;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

use ast::ExprPrinter;
use dialoguer::console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use interpreter::Interpreter;
use io::Read;
use log::error;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use parser::Parser;
use runtime_error::RuntimeError;
use scanner::Scanner;
use structopt::StructOpt;
use token::Token;
use token_type::TokenType;

mod ast;
mod interpreter;
mod object;
mod parser;
mod runtime_error;
mod scanner;
mod token;
mod token_type;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;
static INTERPRETER: Lazy<RwLock<Interpreter>> = Lazy::new(|| RwLock::new(Interpreter::new()));

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
    if unsafe { HAD_RUNTIME_ERROR } {
        process::exit(70);
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

    let mut parser = Parser::new(scanner.scan_tokens());
    let expression = parser.parse();

    if unsafe { HAD_ERROR } {
        return;
    }

    if let Some(expression) = expression {
        INTERPRETER.write().interpret(&expression);
    }
}

fn err(line: usize, message: &str) {
    report(line, "", message);
}

fn err_at(token: &Token, message: &str) {
    if token.kind == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

fn report(line: usize, loc: &str, message: &str) {
    error!("[line {}] Error{}: {}", line, loc, message);
    unsafe { HAD_ERROR = true };
}

fn runtime_error(err: RuntimeError) {
    error!("{}\n[line {}]", err.message(), err.token().line);
    unsafe { HAD_RUNTIME_ERROR = true };
}
