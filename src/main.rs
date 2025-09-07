use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use home::home_dir;
use task::{Task, Tasklist};
use uuid::Uuid;

use crate::task::Status;

pub mod task;

#[derive(Parser, Debug, Clone)]
#[command(name ="todo-cli-rs", version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// add a todo list entry
    Add {
        /// description of the task to add
        description: String,
    },
    /// mark an item as completed
    Complete {
        /// uuid of task to set status
        uuid: Uuid,
    },
    /// list all saved todo list entries
    List,
    /// remove an item
    Remove {
        /// uuid of task to set status
        uuid: Uuid,
    },
}

fn main() {
    let mut savefile: PathBuf;

    match home_dir() {
        Some(path) => {
            savefile = path;
            savefile.push("savefile-todo.json");
        }
        None => {
            println!("Impossible to get your home dir!");
            return;
        }
    }

    let args = Args::parse();

    let mut tasklist = Tasklist::default();

    if fs::exists(&savefile).expect("can not check if file exists") {
        match tasklist.load(&savefile) {
            Ok(_) => {
                println!("(loaded from savefile)");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    match &args.command {
        Command::Add { description } => {
            tasklist.add(Task::new(description));
        }
        Command::Complete { uuid } => match tasklist.edit(*uuid, Status::Complete) {
            Ok(_) => {
                println!("marked task as complete")
            }
            Err(e) => {
                println!("{e}")
            }
        },
        Command::List => {
            println!("{tasklist}");
        }
        Command::Remove { uuid } => {
            tasklist.remove(uuid);
        }
    }

    match tasklist.save(&savefile) {
        Ok(_) => {
            println!("(saved to savefile)");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
