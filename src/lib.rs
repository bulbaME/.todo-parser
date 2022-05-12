mod parser;
mod printer;
mod finder;
mod cli;
mod editor;

use std::collections::HashMap;
use printer::Print;

struct Branch {
    name: String,
    descr: String,
    children: Vec<Branch>
}

type Tree = HashMap<String, Branch>;


fn handle_cli_err(command: &cli::Command) {
    match command {
        cli::Command::Help => printer::print(Print::Help),
        cli::Command::Error(err) => {
            match err {
                cli::Error::EditCommandError => printer::print(Print::Error("Wrong command syntax".to_owned())),
                cli::Error::EditFlagError(s) => printer::print(Print::Error(format!("There is no {} marker", s))),
                cli::Error::FileNotSpecified => printer::print(Print::Error("File wasn't specified after -f flag".to_owned())),
                _ => ()
            }
        },
        _ => ()
    }
}

fn handle_finder_err(e: &finder::Error) {
    use finder::Error;

    match e {
        Error::EmptyTodo => printer::print(Print::Error("No todo files were found".to_owned())),
        Error::FileNotFound(e) => printer::print(Print::Error(format!("File {} not found", e))),
        Error::TooManyTodos(c) => printer::print(Print::Error(format!("{} .todo files were found, but not TODO", c))),
        Error::UnnableToGetTodo => printer::print(Print::Error("Unnable to read directory".to_owned())),
        Error::UnnableToOpenFile => printer::print(Print::Error("Unnable to open todo file".to_owned()))
    }
}

pub fn open() {
    let command = cli::init();

    match command {
        cli::Command::Parse(s) => {
            let found = finder::find(s);
            match found {
                Ok(f) => (),
                Err(e) => handle_finder_err(&e)
            }
        },
        cli::Command::Edit(s, marker, task) => {
            let found = finder::find(s);
            match found {
                Ok(f) => (),
                Err(e) => handle_finder_err(&e)
            }
        },
        _ => handle_cli_err(&command)
    };
}