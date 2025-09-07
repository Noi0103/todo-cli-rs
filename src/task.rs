use std::fmt;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Complete,
    Pending,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = match self {
            Status::Complete => "Complete",
            Status::Pending => "Pending",
        };
        write!(f, "{tmp}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub uuid: Uuid,
    pub description: String,
    pub creation_time: DateTime<Local>,
    pub completion_status: Status,
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task: \n uuid: {}\n description: {}\n creation_time: {}\n completion_status: {}\n",
            self.uuid, self.description, self.creation_time, self.completion_status
        )
    }
}
impl Task {
    pub fn new(description: &str) -> Task {
        Task {
            uuid: Uuid::new_v4(),
            description: description.to_owned(),
            creation_time: Local::now(),
            completion_status: Status::Pending,
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.completion_status = status;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tasklist {
    tasks: Vec<Task>,
}
impl Default for Tasklist {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Display for Tasklist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tmp: String = String::from(">>> all of your tasks <<<");
        for e in &self.tasks {
            tmp = format!("{tmp}\n{e}");
        }
        write!(f, "{tmp}")
    }
}

impl Tasklist {
    pub fn new() -> Tasklist {
        Tasklist { tasks: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn edit(&mut self, uuid: Uuid, status: Status) -> std::result::Result<(), &str> {
        for e in &mut self.tasks {
            if e.uuid == uuid {
                e.completion_status = status;
                return Ok(());
            }
        }
        Err("uuid unknown")
    }

    pub fn remove(&mut self, uuid: &Uuid) {
        self.tasks.retain(|task| task.uuid != *uuid)
    }

    pub fn save(&self, savefile: &PathBuf) -> Result<(), std::io::Error> {
        let json: String = serde_json::to_string(self).expect("parse response json to string");
        std::fs::write(savefile, json)?;
        Ok(())
    }

    pub fn load(&mut self, savefile: &PathBuf) -> Result<(), std::io::Error> {
        let data: Vec<u8> = std::fs::read(savefile)?;
        let parsed: Tasklist =
            serde_json::from_slice(&data).expect("savefile corrupted; can not be parsed");
        *self = parsed.clone();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_task() {
        let mut tasklist = Tasklist::default();
        tasklist.add(Task {
            uuid: Uuid::new_v4(),
            description: String::from("Lorem Ipsum"),
            creation_time: Local::now(),
            completion_status: Status::Complete,
        });
        assert_eq!(1, tasklist.tasks.len())
    }
}
