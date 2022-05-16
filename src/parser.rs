extern crate linked_list;
use linked_list::{LinkedList, Cursor};
use std::iter::FromIterator;

use super::{Tree, Branch, HashMap, Mark};
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum ParseVariant {
    Tab,
    Spaces,
    Default
}

static mut PARSE_V_: ParseVariant = ParseVariant::Default;

pub enum Error {
    ReadFileErr
}

pub fn parse(mut f: File, fl: (bool, bool)) -> Result<Tree, Error> {
    let mut buf: String = String::new();
    if let Err(_) = f.read_to_string(&mut buf) {
        return Err(Error::ReadFileErr);
    }

    let mut tree: Tree = HashMap::new();
    let mut list = LinkedList::from_iter(buf.lines());
    let mut iter = list.cursor();
    loop {
        let line = 
        if let Some(s) = iter.next() { s }
        else { break };

        if let Some(c) = line.chars().nth(0) {
            if c.is_alphanumeric() {
                let name = line.to_owned();
                let branches = parse_branch(&mut iter, 0, fl, Mark::None);
                tree.insert(name, branches);
            }
        }

    }

    Ok(tree)
}

fn parse_branch(iter: &mut Cursor<&str>, depth: usize, fl: (bool, bool), p_mark: Mark) -> Vec<Branch> {
    let mut branches = vec![];
    
    loop {
        let parse_v = unsafe {
            &PARSE_V_
        };

        let line = 
        if let Some(s) = iter.next() {
            if s.trim().is_empty() { break }

            // check how to parse spacings: tabs or spaces
            if let ParseVariant::Default = parse_v {
                if depth == 1 {
                    unsafe {
                        PARSE_V_ = get_parse_v(&s);
                    }
                }
            } 

            s.to_owned()
        } else { 
            iter.prev();
            break 
        };

        // break if new section found
        if let Some(c) = line.chars().nth(0) {
            if c.is_alphanumeric() {
                iter.prev();
                break;
            }
        }

        let skip = if let ParseVariant::Spaces = parse_v { 4 * depth } 
        else { depth };

        if let Some(c) = line.chars().nth(skip) {
            if c != '[' { 
                if let Some(n) = line.find('[') {
                    if n < skip {
                        iter.prev();
                        break
                    }
                }
            }
        } else { 
            iter.prev();
            break 
        }

        // parse line
        let line = line.trim();
        
        // check for right brace
        if let Some(c) = line.chars().nth(2) {
            if c != ']' {
                break
            }
        } else { continue }

        // get mark
        let mark = if let Some(c) = line.chars().nth(1) {
            parse_mark(c)
        } else { continue };

        let mark = match p_mark {
            Mark::Done => Mark::Done,
            Mark::Ignored => Mark::Ignored,
            Mark::Developing => Mark::Developing,
            _ => mark
        };

        match mark {
            Mark::Ignored => if !fl.0 { continue },
            Mark::Developing => if !fl.1 { continue },
            Mark::None => continue,
            _ => ()
        }

        let name = line[3..].trim();
        let mut branch = Branch::new(name, &mark);
        branch.children = parse_branch(iter, depth + 1, fl, mark);
        branches.push(branch);
    }

    branches
}

fn get_parse_v(s: &str) -> ParseVariant {
    let t = if let Some(c) = s.chars().nth(1) { c == '[' }
    else { return ParseVariant::Default };

    if t {
        return ParseVariant::Tab;
    }

    let t = if let Some(c) = s.chars().nth(4) { c == '[' }
    else { return ParseVariant::Default };

    if t {
        ParseVariant::Spaces
    } else {
        ParseVariant::Default
    }
}

fn parse_mark(marker: char) -> Mark {
    match marker {
        'X' => Mark::Done,
        'x' => Mark::Done,
        '/' => Mark::Half,
        ' ' => Mark::Todo,
        '!' => Mark::Ignored,
        '?' => Mark::Developing,
        _ => Mark::None
    }
}