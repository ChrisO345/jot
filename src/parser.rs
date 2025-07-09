use std::fs;

use crate::error;
use crate::jotfile;

pub(crate) struct Parser {
    line: usize,
    num_lines: usize,
}

#[allow(dead_code)]
impl Parser {
    pub(crate) fn new() -> Self {
        Parser {
            line: 0,
            num_lines: 0,
        }
    }

    pub(crate) fn parse(&mut self, jotfile: &mut jotfile::Jotfile) {
        let contents = fs::read_to_string(&jotfile.jotfile).expect("Could not read jotfile");
        self.num_lines = contents.lines().count();

        while let Some(line) = contents.lines().nth(self.line) {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                self.line += 1;
                continue;
            }

            if line.contains(":=") {
                self.parse_var_line(&contents, jotfile);
            } else if line.contains(":") {
                self.parse_task_line(&contents, jotfile);
            } else {
                error::raise_error(&format!(
                    "Invalid line at {}: '{}'. Expected a task definition.",
                    self.line + 1,
                    line
                ));
            }
        }
    }

    fn parse_task_line(&mut self, contents: &str, jotfile: &mut jotfile::Jotfile) {
        let curr_line = contents
            .lines()
            .nth(self.line)
            .unwrap_or_else(|| unreachable!());

        let parts: Vec<&str> = curr_line.splitn(2, ':').collect();
        if parts.len() != 2 {
            error::raise_error(&format!(
                "Invalid task definition at line {}: {}",
                self.line + 1,
                curr_line
            ));
        }

        let task_name = parts[0].trim().to_string();
        let mut command = parts[1].trim().to_string();

        if command.is_empty() {
            self.line += 1;

            while self.line < self.num_lines {
                let next_line = match contents.lines().nth(self.line) {
                    Some(line)
                        if !line.trim().is_empty() && !line.trim_start().starts_with('#') =>
                    {
                        line
                    }
                    _ => {
                        self.line += 1;
                        continue;
                    }
                };

                if next_line.trim_end().ends_with('\\') {
                    command.push_str(" ");
                    command.push_str(next_line.trim_end_matches('\\').trim());
                } else {
                    command.push_str(" ");
                    command.push_str(next_line.trim());
                    self.line += 1;
                    break;
                }

                self.line += 1;
            }

            if command.trim().is_empty() {
                error::raise_error(&format!(
                    "Task '{}' is missing a command at or after line {}",
                    task_name, self.line
                ));
            }
        } else {
            self.line += 1;
        }

        jotfile.tasks.insert(task_name, command.trim().to_string());
    }

    fn parse_var_line(&mut self, contents: &str, jotfile: &mut jotfile::Jotfile) {
        let curr_line = contents
            .lines()
            .nth(self.line)
            .unwrap_or_else(|| unreachable!());

        let parts: Vec<&str> = curr_line.splitn(2, ":=").collect();
        if parts.len() != 2 {
            error::raise_error(&format!(
                "Invalid var definition at line {}: {}",
                self.line + 1,
                curr_line
            ));
        }

        let var_name = parts[0].trim().to_string();
        let command = parts[1].trim().to_string();

        if command.is_empty() {
            error::raise_error(&format!(
                "Variable '{}' is missing a command at line {}",
                var_name,
                self.line + 1
            ));
        }

        jotfile.vars.insert(var_name, command);
        self.line += 1;
    }
}
