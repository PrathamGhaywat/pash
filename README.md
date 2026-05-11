# PASH – Pratham’s Awesome Shell

Pash is an extensible shell written in Rust for personal use and experimentation. It provides a minimal core designed to be extended with custom built-in commands and user-defined functionality. (See overview on how to extend it!)

## Overview

Pash focuses on simplicity and hackability rather than full POSIX compliance. It is intended as a base for building and experimenting with shell features such as custom commands, scripting, and execution behavior.

If you go to src/builtins you will find 3 builtin commands (touch, ls, cd). You can add more by creating new files in that directory and implementing the Builtin trait. Then you can add the command in src/commands.rs to make it available in the shell.

Fully hackable!

## Status

Pash is in active development. Features are incomplete and subject to change. Bugs and breaking changes should be expected.

## Design Goals

* Extensible architecture for custom built-ins
* Lightweight command execution model
* Clear separation between core execution and command logic
* Easy local modification and experimentation

## Usage

Clone the repository and build with Cargo:

```bash
git clone https://github.com/PrathamGhaywat/pash.git
cd pash
cargo run
```

## Note

Pash is not intended as a production-ready shell replacement. It is a development project focused on learning and extensibility.
