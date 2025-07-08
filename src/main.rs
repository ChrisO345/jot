mod cli;
mod jot;

fn main() {
    let cli = cli::parse_args();
    let mut tasks = jot::Jot::new(cli.dir.unwrap());
    tasks.get_tasks_from_jotfile();

    if cli.list {
        tasks.display_tasks();
        return;
    }

    println!("{:?}", tasks);
    println!("Running task: {}", "abc")
}
