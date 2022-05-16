use std::fs;
use std::path::PathBuf;

pub enum Error {
    UnnableToGetTodo,
    EmptyTodo,
    FileNotFound(String),
    TooManyTodos(i32),
    UnnableToOpenFile
}

fn get() -> Option<Vec<PathBuf>> {
    let entries = fs::read_dir(".").ok()?;
    let entries: Vec<PathBuf> = entries
    .filter_map(|x| x.ok())
    .map(|x| x.path())
    .filter_map(|x| {
        let filename = x.file_name()?.to_str()?;
        if filename == "TODO" {
            return Some(x);
        }

        if x.extension()?.to_str()? == ".todo" {
            Some(x)
        } else { None }
    })
    .collect();

    Some(entries)
}

pub fn find(name: Option<String>) -> Result<String, Error> {
    let entries = get().ok_or(Error::UnnableToGetTodo)?;
    if entries.len() == 0 {
        return Err(Error::EmptyTodo);
    }

    let path = if let Some(name) = name {
        entries.iter().find_map(|x| {
            if x.file_name()?.to_str()? == name { Some(x) }
            else { None }
        }).ok_or(Error::FileNotFound(name.to_owned()))?
    } else {
        let c = entries.len();
        if c == 1 {
            entries.first().unwrap()
        } else {
            entries.iter().find_map(|x| {
                if x.file_name()?.to_str()? == "TODO" { Some(x) }
                else { None }
            }).ok_or(Error::TooManyTodos(c as i32))?
        }
    };

    Ok(path.to_str().ok_or(Error::UnnableToOpenFile)?.to_owned())
}