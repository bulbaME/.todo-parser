use super::{cli, finder, parser, printer, editor, Print};

pub fn cli_err(command: &cli::Command) {
    use cli::{Command, Error};

    match command {
        Command::Help => printer::print(Print::Help),
        Command::Error(err) => {
            match err {
                Error::EditCommandError => printer::print(Print::Error("Wrong command syntax".to_owned())),
                Error::EditFlagError(s) => printer::print(Print::Error(format!("There is no {} marker", s))),
                Error::FileNotSpecified => printer::print(Print::Error("File wasn't specified after -f flag".to_owned())),
                _ => ()
            }
        },
        _ => ()
    }
}

pub fn finder_err(e: &finder::Error) {
    use finder::Error;

    match e {
        Error::EmptyTodo => printer::print(Print::Error("No todo files were found".to_owned())),
        Error::FileNotFound(e) => printer::print(Print::Error(format!("File {} not found", e))),
        Error::TooManyTodos(c) => printer::print(Print::Error(format!("{} .todo files were found, but not TODO", c))),
        Error::UnnableToGetTodo => printer::print(Print::Error("Unnable to read directory".to_owned())),
        Error::UnnableToOpenFile => printer::print(Print::Error("Unnable to open todo file".to_owned()))
    }
}

pub fn parser_err(e: &parser::Error) {
    use parser::Error;

    match e {
        Error::ReadFileErr => printer::print(Print::Error("Unnable to read file".to_owned())),
    }
}

pub fn editor_err(e: &editor::Error) {
    use editor::Error;

    match e {
        Error::NoMatch(s) => printer::print(Print::Error(format!("No matches found for task named {}", s))),
        Error::TooManyMatches(s, n) => printer::print(Print::Error(format!("Found {} matches for task named {}", n, s))),
        Error::WriteError(s) => printer::print(Print::Error(format!("Unnable to write to the file: {}", s)))
    }
}