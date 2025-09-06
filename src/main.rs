use clap::{Parser, Subcommand};
use std::path::PathBuf;
use uuid::Uuid;

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
        #[arg(short, long)]
        description: String,
    },
    /// mark an item as completed
    Complete {
        /// uuid of task to set status
        #[arg(short, long)]
        uuid: Uuid,
    },
    /// list all saved todo list entries
    List {},
    /// remove an item
    Remove {
        /// uuid of task to set status
        #[arg(short, long)]
        uuid: Uuid,
    },
}

fn main() {
    use task::{Task, Tasklist};

    // hard coded arg
    let savefile: PathBuf = PathBuf::from("./save.json");

    let args = Args::parse();

    let mut tasklist = Tasklist::default();
    let _ = tasklist.load(&savefile);

    match &args.command {
        Command::Add { description } => {
            tasklist.add(Task::new(description));
        }
        Command::Complete { uuid } => {
            // TODO
            println!("{uuid}");
        }
        Command::List {} => {
            println!("{tasklist:#?}");
        }
        Command::Remove { uuid } => {
            tasklist.remove(uuid);
        }
    }

    let _ = tasklist.save(&savefile);
}
