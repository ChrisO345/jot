use std::fs;

use crate::{error, jotfile};

pub(crate) struct Parser {
    lines: Vec<String>,
    line_index: usize,
    current_section: Option<String>,
}

#[allow(dead_code)]
impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            lines: Vec::new(),
            line_index: 0,
            current_section: None,
        }
    }

    pub(crate) fn parse(&mut self, jotfile: &mut jotfile::Jotfile) {
        let content = fs::read_to_string(&jotfile.jotfile).unwrap_or_else(|_| unreachable!());

        self.lines = content.lines().map(str::to_string).collect();

        while self.line_index < self.lines.len() {
            let line = &self.lines[self.line_index];
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                self.line_index += 1;
                continue;
            }

            if trimmed.contains(":=") {
                self.parse_var_line(jotfile);
            } else if trimmed.contains('=') && !trimmed.contains(":=") {
                self.parse_section_line(jotfile);
            } else if trimmed.contains(':') && !trimmed.contains(":=") {
                self.parse_task_line(jotfile);
            } else {
                error::raise_error(&format!(
                    "Invalid line at {}: '{}'. Expected a task, variable, or section definition.",
                    self.line_index + 1,
                    line
                ));
            }
        }
    }

    fn parse_var_line(&mut self, jotfile: &mut jotfile::Jotfile) {
        let line = &self.lines[self.line_index];
        let parts: Vec<&str> = line.splitn(2, ":=").collect();

        if parts.len() != 2 {
            error::raise_error(&format!(
                "Invalid variable definition at line {}: {}",
                self.line_index + 1,
                line
            ));
        }

        let var_name = parts[0].trim();
        let command = parts[1].trim();

        if command.is_empty() {
            error::raise_error(&format!(
                "Variable '{}' is missing a value at line {}",
                var_name,
                self.line_index + 1
            ));
        }

        jotfile
            .vars
            .insert(var_name.to_string(), command.to_string());
        self.line_index += 1;
    }

    fn parse_section_line(&mut self, jotfile: &mut jotfile::Jotfile) {
        let line = &self.lines[self.line_index];
        let parts: Vec<&str> = line.splitn(2, '=').collect();

        if parts.len() != 2 {
            error::raise_error(&format!(
                "Invalid section definition at line {}: {}",
                self.line_index + 1,
                line
            ));
        }

        let section_name = parts[1].trim().trim_matches('"');
        if section_name.is_empty() {
            error::raise_error(&format!(
                "Section name missing after '=' at line {}",
                self.line_index + 1
            ));
        }

        jotfile
            .sections
            .entry(section_name.to_string())
            .or_default();
        self.current_section = Some(section_name.to_string());
        self.line_index += 1;
    }

    fn parse_task_line(&mut self, jotfile: &mut jotfile::Jotfile) {
        let line = &self.lines[self.line_index];
        let (task_name, command) = self.split_task_line(line);

        let multi: Vec<String>;
        if command.trim().is_empty() {
            multi = self.collect_multiline_command();

            if multi.is_empty() {
                error::raise_error(&format!(
                    "Task '{}' is missing a command at or after line {}",
                    task_name,
                    self.line_index + 1
                ));
            }

            jotfile.tasks.insert(task_name.clone(), multi);
        } else {
            self.line_index += 1;

            jotfile
                .tasks
                .insert(task_name.clone(), vec![command.trim().to_string()]);
        }

        if let Some(section) = &self.current_section {
            jotfile
                .sections
                .entry(section.clone())
                .or_default()
                .push(task_name);
        }
    }

    fn split_task_line(&self, line: &str) -> (String, String) {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            error::raise_error(&format!(
                "Invalid task definition at line {}: {}",
                self.line_index + 1,
                line
            ));
        }
        (parts[0].trim().to_string(), parts[1].trim().to_string())
    }

    fn collect_multiline_command(&mut self) -> Vec<String> {
        let mut command: Vec<String> = Vec::new();

        self.line_index += 1;

        while self.line_index < self.lines.len() {
            let line = &self.lines[self.line_index];
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                self.line_index += 1;
                continue;
            }

            if trimmed.ends_with(';') {
                command.push(trimmed.trim_end_matches(';').trim().to_string());
                self.line_index += 1;
                break;
            } else {
                command.push(trimmed.to_string());
            }

            self.line_index += 1;
        }

        command
    }
}
