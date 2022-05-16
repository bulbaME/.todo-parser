extern crate termcolor;

use super::{Tree, Branch, Mark};
use std::collections::HashSet;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum Print {
    Error(String),
    Tree(Tree),
    Help,
    Edited(Mark, String)
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

    output("\tin the development state: ", &yellow);
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

fn print_tree(t: Tree) {
    let mut title = ColorSpec::new();
    title.set_bold(true);
    let mut green = ColorSpec::new();
    green.set_bold(true);
    green.set_fg(Some(Color::Green));
    let mut yellow = ColorSpec::new();
    yellow.set_bold(true);
    yellow.set_fg(Some(Color::Yellow));
    let mut red = ColorSpec::new();
    red.set_bold(true);
    red.set_fg(Some(Color::Red));

    for (name, branches) in t {
        output(&format!("{}\n", name), &title);
        branches.iter().enumerate().for_each(|(n, b)| {
            let is_last = n + 1 == branches.len();
            let mut pipes = HashSet::new();
            print_branch(b, 0, is_last, &mut pipes);
        });
        println!();
    }
}

fn print_branch(branch: &Branch, depth: usize, is_last: bool, pipes: &mut HashSet<usize>) {
    let mut title = ColorSpec::new();
    title.set_bold(true);
    let mut green = ColorSpec::new();
    green.set_bold(true);
    green.set_fg(Some(Color::Green));
    let mut yellow = ColorSpec::new();
    yellow.set_bold(true);
    yellow.set_fg(Some(Color::Yellow));
    let mut red = ColorSpec::new();
    red.set_bold(true);
    red.set_fg(Some(Color::Red));

    let ch = 
    if is_last { "└──" } 
    else { "├──" };

    if depth > 0 {
        let mut pre = format!("{} {}", std::iter::repeat(" ").take((depth-1) * 4).collect::<String>(), ch);
        let mut pipes_lst: Vec<usize> = pipes.iter().map(|x| *x).collect();
        pipes_lst.sort();

        for i in pipes_lst.into_iter().rev() {
            if i >= depth {
                continue;
            }

            let i = (i - 1) * 4 + 1;
            pre = format!("{}│{}", &pre[..i], &pre[i+1..]);
        }

        output(&pre, &title);
        if !is_last {
            pipes.insert(depth);
        } else {
            pipes.remove(&depth);
        }
    }

    output("[", &title);
    match branch.marker {
        Mark::Done => output("X", &green),
        Mark::Half => output("/", &yellow),
        Mark::Ignored => output("!", &red),
        Mark::Developing => output("?", &red),
        _ => output(" ", &title),
    }
    output(&format!("] {}\n", branch.name), &title);

    branch.children.iter().enumerate().for_each(|(n, b)| {
        let is_last = n + 1 == branch.children.len();
        print_branch(b, depth + 1, is_last, pipes);
    });
}

fn print_edited(task: &str, mark: &Mark) {
    let mut title = ColorSpec::new();
    title.set_bold(true);
    let mut green = ColorSpec::new();
    green.set_bold(true);
    green.set_fg(Some(Color::Green));
    let mut blue = ColorSpec::new();
    blue.set_bold(true);
    blue.set_fg(Some(Color::Blue));

    output("Task ", &title);
    output(task, &blue);
    output(" marked as ", &title);
    output({ match mark {
        Mark::Developing => "under development",
        Mark::Done => "done",
        Mark::Half => "in process",
        Mark::Ignored => "ignored",
        Mark::Todo => "todo",
        _ => ""
    } }, &green);

    println!();
}

pub fn print(p: Print) {
    match p {
        Print::Help => print_help(),
        Print::Error(e) => print_err(&e),
        Print::Tree(t) => print_tree(t),
        Print::Edited(m, t) => print_edited(&t, &m)
    }

    output("\n", &ColorSpec::default());
}