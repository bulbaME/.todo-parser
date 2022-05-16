use super::{Tree, Branch, Mark, error_handler};
use std::fs::File;
use std::io::Write;

pub enum Error {
    TooManyMatches(String, i32),
    NoMatch(String),
    WriteError(String)
}

pub fn edit(tree: Tree, f: File, marker: &Mark, task: &String) -> Result<Tree, Error> {
    let mut tree_new = Tree::new();

    let mut c = 0;
    for (_, branches) in tree.iter() {
        for branch in branches {
            c += search_branch(branch, &task.to_lowercase());
        }
    }

    if c == 0 {
        return Err(Error::NoMatch(task.to_owned()));
    } else if c > 1 {
        return Err(Error::TooManyMatches(task.to_owned(), c));
    }

    for (name, branches) in tree.iter() {
        let mut branches_new = vec![];
        for branch in branches {
            branches_new.push(edit_branch(branch, &marker, &task.to_lowercase()));
        }
        tree_new.insert(name.to_owned(), branches_new);
    }

    write(f, &tree_new);
    Ok(tree_new)
}

fn write(f: File, tree: &Tree) {
    let mut result = String::new();
    for (name, branches) in tree.iter() {
        result += &format!("{}\n", name);
        for b in branches.iter() {
            result += &format_branch(b, 0);
        }
        result += "\n";
    }

    let mut f = f;
    if let Err(e) = write!(&mut f, "{}", result) {
        error_handler::editor_err(&Error::WriteError(e.to_string()));
    } else {

    }
}

fn format_branch(branch: &Branch, depth: usize) -> String {
    let mut result = format!("{}[{}] {}\n", std::iter::repeat("\t").take(depth).collect::<String>(), {
        match branch.marker {
            Mark::Done => 'X',
            Mark::Half => '/',
            Mark::Developing => '?',
            Mark::Ignored => '!',
            _ => ' '
        }
    }, branch.name);

    for b in branch.children.iter() {
        result += &format_branch(b, depth + 1);
    }

    result
}

fn search_branch(branch: &Branch, task: &str) -> i32 {
    let mut c = 0;
    if let Some(_) = branch.name.to_lowercase().find(task) { c += 1 }
    branch.children.iter().for_each(|b| c += search_branch(b, task));

    c
}

fn edit_branch(branch: &Branch, marker: &Mark, task: &str) -> Branch {
    let mut branch_new = Branch::new(&branch.name, &branch.marker);

    if let Some(_) = branch.name.to_lowercase().find(task) {
        branch_new.marker = marker.clone();
        for b in branch.children.iter() {
            branch_new.children.push(update_branch(&b, marker));
        }
    } else {
        for b in branch.children.iter() {
            branch_new.children.push(edit_branch(&b, marker, task));
        }
    }

    branch_new
}

fn update_branch(branch: &Branch, marker: &Mark) -> Branch {
    let mut branch_new = Branch::new(&branch.name, &branch.marker);
    branch_new.marker = match marker {
        Mark::Developing => Mark::Developing,
        Mark::Done => Mark::Done,
        Mark::Ignored => Mark::Ignored,
        _ => branch_new.marker
    };

    for b in branch.children.iter() {
        branch_new.children.push(update_branch(b, marker));
    }

    branch_new
}