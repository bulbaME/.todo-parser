extern crate termcolor;

use super::{Tree, Branch};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum Print {
    Error(String),
    Tree(Tree),
    Help
}

#[allow(unused_must_use)]
fn output(text: &str, color_spec: &ColorSpec) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(color_spec);
    write!(&mut stdout, "{}", text);
}

fn print_help() {
    let default = ColorSpec::default();
    let mut title = ColorSpec::new();
    title.set_bold(true);
    let mut green = ColorSpec::new();
    green.set_bold(true);
    green.set_fg(Some(Color::Green));
    let mut yellow = ColorSpec::new();
    yellow.set_bold(true);
    yellow.set_fg(Some(Color::Yellow));

    // about
    output(".todo-parser by BulbaME", &title);
    print!("\n");
    output("todo files parser/reader", &default);
    print!("\n\n");

    // usage
    output("Usage: ", &title);
    output("todo [<file name>] [<flag> <command>]", &default);
    print!("\n");

    // flags
    output("Flags: \n", &title);

    // help
    output("\t-h : ", &title);
    output("help", &default);
    print!("\n");

    // file name
    output("\t-f : ", &title);
    output("<", &default);
    output("file name", &green);
    output(">", &default);
    print!("\n");

    // command
    output("\t-c : ", &title);

    // task
    output("<", &default);
    output("task", &green);
    output("> mark as <", &default);
    output("marker", &green);
    output(">", &default);

    print!("\n");

    // markers
    output("Markers: \n", &title);

    output("\tdone: ", &yellow);
    output("done / D\n", &title);

    output("\tin process: ", &yellow);
    output("doing / H\n", &title);

    output("\ttodo: ", &yellow);
    output("todo / N\n", &title);

    output("\tignored: ", &yellow);
    output("ignored / I\n", &title);

    output("\tin process of planning: ", &yellow);
    output("planning / P\n", &title);
}

fn print_err(text: &str) {
    let mut red = ColorSpec::new();
    red.set_bold(true);
    red.set_fg(Some(Color::Red));
    let mut title = ColorSpec::new();
    title.set_bold(true);

    output("Error occured!\n", &red);
    output(text, &title);
    output("\n\nUse -h flag for help\n", &title)
}

fn print_tree() {

}

pub fn print(p: Print) {
    print!("\n");

    match p {
        Print::Help => print_help(),
        Print::Error(e) => print_err(&e),
        _ => ()
    }

    print!("\n");
}