use std::collections::HashMap;
use std::{fs, path::PathBuf};

use crate::{error, parser};

use owo_colors::OwoColorize;

#[derive(Debug)]
pub(crate) struct Jotfile {
    pub(crate) dir: PathBuf,
    pub(crate) jotfile: PathBuf,
    pub(crate) tasks: HashMap<String, String>,

    pub(crate) vars: HashMap<String, String>,
    pub(crate) overrides: HashMap<String, String>,
}

impl Jotfile {
    pub(crate) fn new(
        dir: Option<PathBuf>,
        jotfile: Option<String>,
        shell: Option<String>,
    ) -> Self {
        let dir = dir.map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        let mut jotfile = Jotfile {
            dir: dir.clone(),
            jotfile: dir.join(jotfile.unwrap_or_else(|| "jotfile".to_string())),
            tasks: HashMap::new(),

            vars: HashMap::new(),
            overrides: HashMap::new(),
        };

        jotfile.overrides.insert(
            "shell".to_string(),
            shell.unwrap_or_else(|| std::env::var("SHELL").unwrap_or_else(|_| "".to_string())),
        );

        return jotfile;
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
            error::raise_warning("No tasks found in the jotfile. Please add some tasks to run.");
            return;
        }

        // TODO: Improve the display format
        println!("{}:", "Available tasks".bold().underline());
        for (task, command) in &self.tasks {
            println!("{}: {}", task.bold(), command);
        }
    }

    pub(crate) fn execute_task(&self, task: &str) {
        if let Some(command) = self.get_task(task) {
            if let Err(e) = std::env::set_current_dir(&self.dir) {
                error::raise_error(&format!(
                    "Failed to change directory to '{}': {}",
                    self.dir.display(),
                    e
                ));
                return;
            }

            let shell = self
                .overrides
                .get("shell")
                .map(|s| s.as_str())
                .or_else(|| self.vars.get("shell").map(|s| s.as_str()))
                .unwrap_or_else(|| unreachable!());

            let status = std::process::Command::new(shell)
                .arg("-c")
                .arg(command)
                .status();
            match status {
                Ok(_) => {}
                Err(e) => {
                    error::raise_error(&format!(
                        "Failed to execute task '{}': {}",
                        task.bold().yellow(),
                        e
                    ));
                }
            }
        } else {
            error::raise_error(&format!(
                "Task '{}' not found in the jotfile.",
                task.bold().yellow()
            ));
        }
    }

    pub(crate) fn fill_default_options(&mut self) {
        if !self.vars.contains_key("shell") {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
            self.vars.insert("shell".to_string(), shell);
        }
    }
}
