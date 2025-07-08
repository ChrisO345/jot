use std::collections::HashMap;
use std::{fs, path::PathBuf};

use owo_colors::OwoColorize;

#[derive(Debug)]
pub(crate) struct Jot {
    pub _dir: PathBuf,
    pub jotfile: PathBuf,
    pub tasks: HashMap<String, String>,
}

#[allow(dead_code)]
impl Jot {
    pub(crate) fn new(dir: PathBuf) -> Self {
        Jot {
            _dir: dir.clone(),
            jotfile: dir.join("jotfile"),
            tasks: HashMap::new(),
        }
    }

    pub(crate) fn validate_jotfile_path(&self) -> bool {
        if let Ok(does_exist) = fs::exists(&self.jotfile) {
            if does_exist {
                return true;
            } else {
                println!(
                    "{}",
                    "Could not find jotfile in the specified directory".red()
                );
            }
        } else {
            println!("{}", "Error checking for jotfile existence".red());
        }

        return false;
    }

    pub(crate) fn get_tasks_from_jotfile(&mut self) {
        if !self.validate_jotfile_path() {
            return;
        }

        let contents = fs::read_to_string(&self.jotfile).expect("Could not read jotfile");
        for line in contents.lines() {
            if let Some((task, command)) = line.split_once(':') {
                self.tasks
                    .insert(task.trim().to_string(), command.trim().to_string());
            }
        }
    }

    pub(crate) fn get_task(&self, task: &str) -> Option<&String> {
        self.tasks.get(task)
    }

    pub(crate) fn display_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks found in the jotfile".yellow());
            return;
        }

        println!("{}:", "Available tasks".bold().underline());
        for (task, command) in &self.tasks {
            println!("{}: {}", task.bold(), command);
        }
    }
}
