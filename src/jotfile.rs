use std::collections::HashMap;
use std::{fs, path::PathBuf};

use owo_colors::OwoColorize;

use crate::{error, parser};

#[derive(Debug)]
pub(crate) struct Jotfile {
    pub(crate) dir: PathBuf,
    pub(crate) jotfile: PathBuf,
    pub(crate) tasks: HashMap<String, Vec<String>>,
    pub(crate) vars: HashMap<String, String>,
    pub(crate) overrides: HashMap<String, String>,
    pub(crate) sections: HashMap<String, Vec<String>>,
}

impl Jotfile {
    pub(crate) fn new(
        dir: Option<PathBuf>,
        jotfile: Option<String>,
        shell: Option<String>,
    ) -> Self {
        let dir = dir.unwrap_or_else(|| PathBuf::from("."));
        let jotfile_path = dir.join(jotfile.unwrap_or_else(|| "jotfile".to_string()));

        let mut jotfile = Jotfile {
            dir: dir.clone(),
            jotfile: jotfile_path,
            tasks: HashMap::new(),
            vars: HashMap::new(),
            overrides: HashMap::new(),
            sections: HashMap::new(),
        };

        jotfile.overrides.insert(
            "shell".to_string(),
            shell
                .or_else(|| std::env::var("SHELL").ok())
                .unwrap_or_else(|| "".to_string()),
        );

        jotfile
    }

    pub(crate) fn validate_jotfile_path(&self) -> bool {
        if let Ok(exists) = fs::metadata(&self.jotfile) {
            if exists.is_file() {
                return true;
            } else {
                println!("{}", "Jotfile path is not a file.".red());
            }
        } else {
            println!("{}", "Error checking jotfile existence.".red());
        }
        false
    }

    pub(crate) fn get_tasks_from_jotfile(&mut self) {
        if self.validate_jotfile_path() {
            let mut parser = parser::Parser::new();
            parser.parse(self);
        }
    }

    pub(crate) fn get_task(&self, name: &str) -> &Vec<String> {
        self.tasks.get(name).unwrap_or_else(|| unreachable!())
    }

    pub(crate) fn display_tasks(&self) {
        if self.tasks.is_empty() {
            error::raise_warning("No tasks found in the jotfile. Please add some tasks.");
            return;
        }

        println!("{}:", "Jotfile".bold().underline().green());

        for (task, _) in &self.tasks {
            if !self.sections.values().any(|tasks| tasks.contains(task)) {
                self.print_task(task);
            }
        }

        if !self.sections.is_empty() {
            for (section, tasks) in &self.sections {
                println!("\n{}:", section.bold().underline().purple());
                for task in tasks {
                    self.print_task(task);
                }
            }
        }
    }

    fn print_task(&self, task: &str) {
        println!("  {}:", task.bold().yellow());
        for cmd in self.get_task(task) {
            println!("    {}", cmd);
        }
    }

    pub(crate) fn execute_task(&self, task: &str) {
        let cmds = self.get_task(task);

        if let Err(e) = std::env::set_current_dir(&self.dir) {
            error::raise_error(&format!(
                "Failed to change directory to '{}': {}",
                self.dir.display(),
                e
            ));
        }

        let shell = self
            .overrides
            .get("shell")
            .or_else(|| self.vars.get("shell"))
            .map(|s| s.as_str())
            .unwrap_or_else(|| unreachable!());

        for command in cmds {
            let status = std::process::Command::new(shell)
                .arg("-c")
                .arg(command)
                .status();

            if let Err(e) = status {
                error::raise_error(&format!(
                    "Failed to execute task '{}': {}",
                    task.bold().yellow(),
                    e
                ));
            }
        }
    }

    pub(crate) fn fill_default_options(&mut self) {
        if !self.vars.contains_key("shell") {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
            self.vars.insert("shell".to_string(), shell);
        }
    }
}
