mod err;
use err::{ParseErr, ReadErr};

pub use json::{parse, stringify};
pub use std::error::Error;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut f = File::open(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let mut contents = String::new();
        f.read_to_string(&mut contents).map_err(|e| err::ReadErr {
            child_err: Box::new(e),
        })?;

        let json = json::parse(&contents).map_err(|e| err::ParseErr::Malformed(Box::new(e)))?;

        let title = json["title"].as_str().ok_or(ParseErr::Empty)?;
        let tasks: Vec<_> = json["tasks"]
            .members()
            .map(|task| Task {
                id: task["id"].as_u32().unwrap(),
                description: task["description"].as_str().unwrap().to_string(),
                level: task["level"].as_u32().unwrap(),
            })
            .collect();

        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList {
            title: title.to_string(),
            tasks,
        })
    }
}
