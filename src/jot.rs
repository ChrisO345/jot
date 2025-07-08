use super::*;

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "jot",
    version = "0.0.1",
    long_about = r#"jot is a simple command-runner inspired by task runners like make, but more lightweight.

It reads tasks from a plain text "jotfile" and allows you to run them easily from the command line.

Examples:
  jot build        # Runs the "build" task from the jotfile
  jot --list       # Lists all available tasks

By default, jot looks for a file named "jotfile" in the current directory, unless you specify another directory with --dir."#,
    arg_required_else_help = true // TODO: this might want to be changed to list tasks if no task is specified.
)]
pub(crate) struct Jot {
    /// The task to run
    #[arg(value_name = "TASK")]
    pub task: Option<String>,

    /// List all tasks in the jotfile
    #[arg(short, long)]
    pub list: bool,

    /// The directory to find the jotfile in
    #[arg(short, long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Override the jotfile filename
    #[arg(short, long, value_name = "JOTFILE")]
    pub jotfile: Option<String>,

    /// Override the shell to use for executing tasks
    #[arg(short, long, value_name = "SHELL")]
    pub shell: Option<String>,
}

pub(crate) fn run() {
    let cli = Jot::parse();

    let mut jotfile = jotfile::Jotfile::new(cli.dir, cli.jotfile, cli.shell);
    jotfile.get_tasks_from_jotfile();

    if cli.list {
        jotfile.display_tasks();
        return;
    }

    if let Some(task) = cli.task {
        jotfile.execute_task(&task);
        return;
    }

    unreachable!()
}
