# jot

> [!WARNING]
> Jot is still in early stages of development. There will be breaking changes as features are added and improved.

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

# Alternatively you can install jot using jot
cargo run --release -- bin
```

---

## Usage

Here's a quick example of how to use `jot`:

```bash
# Run a task named "build" defined in your jotfile
jot build

# List all tasks defined in the jotfile
jot --list

# List the help message
jot --help
```

---

## Jotfile Format

**jot is still in early stages of development. This section will be updated as features are added.**

- A new command is defined in a jotfile using the following format:
```jot
bin:
  cp target/release/jot $CARGO_HOME/bin/jot;
```

- You can define sections in a jotfile to group commands. Any line starting with `=` is considered to be a section header.
```jot
= Cargo Commands
build:
  cargo build --release;

test:
  cargo test;
```

- Lines starting with `#` are comments and will be ignored by `jot`:
```jot
# This is a comment
= Example Section
example:
  echo "This is an example command";
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
