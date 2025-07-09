use owo_colors::OwoColorize;

pub(crate) fn raise_error(message: &str) {
    eprintln!("{}: {}", "Error".red().bold(), message);
    std::process::exit(1);
}

pub(crate) fn raise_warning(arg: &str) {
    eprintln!("{}: {}", "Warning".yellow().bold(), arg);
    std::process::exit(0);
}
