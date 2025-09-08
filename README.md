# todo-cli-rs
A simple todo application written in rust and packaged with nix. 

# Table of contents
- [overview](#overview)
- [using nix commands](#using-nix-commands)
  - [run](#run)  
  - [build](#build)
  - [developer shell](#developer-shell)

# overview
```
Usage: todo-cli-rs <COMMAND>

Commands:
  add       add a todo list entry
  complete  mark an item as completed
  list      list all saved todo list entries
  remove    remove an item
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# using nix commands
## run
run the application ad-hoc with nix:
```
nix run github:Noi0103/todo-cli-rs\#default -- help
```
```
nix run --extra-experimental-features nix-command --extra-experimental-features flakes github:Noi0103/todo-cli-rs\#default -- help
```

## build
using nix _and_ having flakes feature enabled
```
nix build
```
```
nix build --extra-experimental-features nix-command --extra-experimental-features flakes
```

## developer shell
using nix _and_ having flakes feature enabled
```
nix develop
```
```
nix develop --option experimental-features flakes --extra-experimental-features nix-command
```
(Note: see direnv for QoL with devshells)

inside the development shell use cargo commands (instead of nix) to avoid fully rebuilding on each change `cargo run -- help` (see cargo language book or some similar resource)