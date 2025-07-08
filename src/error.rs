use owo_colors::OwoColorize;

pub(crate) fn raise_error(message: &str) {
    eprintln!("{}: {}", "Error".red().bold(), message);
    std::process::exit(1);
}
