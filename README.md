# jot

`jot` is a lightweight, fast, and easy-to-use command runner for tasks defined in a plain text jotfile. Designed for simplicity and developer productivity, it lets you define custom commands and run them effortlessly from the command line.

---

## Features

* Runs custom tasks defined in a plain-text jotfile
* Supports listing available tasks
* Works out of the box with zero configuration
* Flexible directory support to locate your jotfile
* Minimal dependencies, fast startup time

---

## Installation

`jot` can be installed by cloning the repository and building:

```bash
git clone https://github.com/chriso345/jot.git
cd jot
cargo build --release

# It is recommend to copy the binary to a directory in your PATH
cp target/release/jot $CARGO_HOME/bin/jot
```

---

## Usage

Here's a quick example of how to use `jot`:

```bash
# Run a task named "build" defined in your jotfile
jot build

# List all tasks defined in the jotfile
jot --list

# Specify a different directory containing your jotfile
jot --dir /path/to/project build
```

---

## Jotfile Format

**jot is still in early stages of development. This section will be updated as features are added.**

Lines starting with `#` are comments and ignored.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
