use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "jot",
    version = "0.0.1",
    about = "A simple lightweight task runner",
    long_about = r#"jot is a simple command-runner inspired by task runners like make, but more lightweight.

It reads tasks from a plain text "jotfile" and allows you to run them easily from the command line.

Examples:
  jot build        # Runs the "build" task from the jotfile
  jot --list       # Lists all available tasks

By default, jot looks for a file named "jotfile" in the current directory, unless you specify another directory with --dir."#
)]
pub(crate) struct CLI {
    /// The task to run
    #[arg(value_name = "TASK", required_unless_present = "list")]
    pub task: Option<String>,

    /// List all tasks in the jotfile
    #[arg(short, long)]
    pub list: bool,

    /// The directory to find the jotfile in
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub dir: Option<PathBuf>,
}

pub(crate) fn parse_args() -> CLI {
    CLI::parse()
}
