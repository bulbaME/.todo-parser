mod parser;
mod printer;
mod finder;
mod cli;
mod editor;
mod error_handler;

use std::collections::HashMap;
use printer::Print;
use std::fs::{OpenOptions, File};

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub marker: Mark,
    pub children: Vec<Branch>,
}

impl Branch {
    pub fn new(name: &str, marker: &Mark) -> Branch {
        Branch {
            name: name.to_owned(),
            marker: marker.clone(),
            children: vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mark {
    Done,
    Half,
    Todo,
    Ignored,
    Developing,
    None
}

type Tree = HashMap<String, Vec<Branch>>;


pub fn open() {
    let command = cli::init();

    match command {
        cli::Command::Parse(s, fl) => {
            let found = finder::find(s);
            match found {
                Ok(f_path) => {
                    if let Ok(f) = File::open(&f_path) {
                        match parser::parse(f, fl) {
                            Ok(tree) => printer::print(Print::Tree(tree)),
                            Err(e) => error_handler::parser_err(&e)
                        }
                    } else {
                        error_handler::finder_err(&finder::Error::UnnableToOpenFile);
                    }
                },
                Err(e) => error_handler::finder_err(&e)
            }
        },
        cli::Command::Edit(s, marker, task) => {
            let found = finder::find(s);
            match found {
                Ok(f_path) => {
                    if let Ok(f) = OpenOptions::new().write(true).open(&f_path) {
                        match parser::parse({
                                if let Ok(f) = File::open(&f_path) { f }
                                else {
                                    error_handler::parser_err(&parser::Error::ReadFileErr);
                                    return;
                                }
                            }, (true, true)) {
                            Ok(tree) => {
                                if let Err(e) = editor::edit(tree, f, &marker, &task) {
                                    error_handler::editor_err(&e);
                                } else {
                                    printer::print(Print::Edited(marker, task));
                                }
                            },
                            Err(e) => error_handler::parser_err(&e)
                        }
                    } else {
                        error_handler::finder_err(&finder::Error::UnnableToOpenFile);
                    }
                },
                Err(e) => error_handler::finder_err(&e)
            }
        },
        _ => error_handler::cli_err(&command)
    };
}