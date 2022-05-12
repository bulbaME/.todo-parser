use std::env;

pub type FileName = Option<String>;

#[derive(Debug)]
pub enum Mark {
    Done,
    Half,
    Todo,
    Ignored,
    Developing,
    None
}

#[derive(Debug)]
pub enum Error {
    FileNotSpecified,
    EditFlagError(String),
    EditCommandError,
    _CommandNotSpecified,
}

#[derive(Debug)]
pub enum Command {
    Help,
    Parse(FileName),
    Error(Error),
    Edit(FileName, Mark, String)
}

pub fn init() -> Command {
    let args: Vec<String> = env::args().collect();
    if let None = args.get(1) {
        return Command::Parse(None);
    }

    let mut file: FileName = None;

    if args.len() >= 2 {
        match &args[1][..] {
            "-h" => return Command::Help,
            "-c" => (),
            "-f" => (),
            s => {
                if args.len() == 2 {
                    return Command::Parse(Some(s.to_owned()));    
                } else {
                    file = Some(s.to_owned());
                }
            }
        }
    }

    let mut action: (Mark, String) = (Mark::None, String::new());

    for i in 1..args.len() {
        if args[i] == "-f" {
            if let Some(filename) = args.get(i+1) {
                file = Some(filename.to_owned());
            } else {
                return Command::Error(Error::FileNotSpecified);
            }
        }
    
        if args[i] == "-c" {
            if args.len() < i + 5 {
                return Command::Error(Error::EditCommandError);
            }
            
            action.0 = Mark::None;
            action.1.clear();

            if let Some(task) = args.get(i+1) {
                if args[i+2] != "mark" || args[i+3] != "as" {
                    return Command::Error(Error::EditCommandError);
                }

                action.1 = task.to_owned();
                action.0 = match &args[i+4][..] {
                    "done" => Mark::Done,
                    "ignored" => Mark::Ignored,
                    "planning" => Mark::Developing,
                    "todo" => Mark::Todo,
                    "doing" => Mark::Half,
                    
                    "D" => Mark::Done,
                    "I" => Mark::Ignored,
                    "P" => Mark::Developing,
                    "N" => Mark::Todo,
                    "H" => Mark::Half,

                    c => return Command::Error(Error::EditFlagError(c.to_owned()))
                };
            }
        }
    }

    if let Mark::None = action.0 {
        Command::Parse(file)
    } else {
        Command::Edit(file, action.0, action.1)
    }
}