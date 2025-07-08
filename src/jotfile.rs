use super::*;

use std::collections::HashMap;
use std::{fs, path::PathBuf};

use owo_colors::OwoColorize;

#[derive(Debug)]
pub(crate) struct Jotfile {
    pub(crate) dir: PathBuf,
    pub(crate) jotfile: PathBuf,
    pub(crate) tasks: HashMap<String, String>,

    pub(crate) shell: String,
}

impl Jotfile {
    pub(crate) fn new(
        dir: Option<PathBuf>,
        jotfile: Option<String>,
        shell: Option<String>,
    ) -> Self {
        let dir = dir.map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        Jotfile {
            dir: dir.clone(),
            jotfile: dir.join(jotfile.unwrap_or_else(|| "jotfile".to_string())),
            tasks: HashMap::new(),

            shell: shell
                .unwrap_or_else(|| std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())),
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

        let mut parser = parser::Parser::new();
        parser.parse(self);
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

    pub(crate) fn execute_task(&self, task: &str) {
        if let Some(command) = self.get_task(task) {
            if let Err(e) = std::env::set_current_dir(&self.dir) {
                eprintln!(
                    "{}",
                    format!(
                        "Failed to change directory to '{}': {}",
                        self.dir.display(),
                        e
                    )
                    .red()
                );
                return;
            }

            let status = std::process::Command::new(&self.shell)
                .arg("-c")
                .arg(command)
                .status();
            match status {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "{}",
                        format!("Failed to execute task '{}': {}", task.bold(), e).red()
                    );
                }
            }
        } else {
            eprintln!(
                "{}",
                format!("Task '{}' not found in the jotfile.", task.bold()).red()
            );
        }
    }
}
