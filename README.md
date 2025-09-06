# todo-cli-rs
```
Usage: todo-cli-rs [OPTIONS] <COMMAND>

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


## build
using nix _and_ having flakes feature enabled
```sh
nix build
```

using default nix
```sh
nix build --option experimental-features flakes --extra-experimental-features nix-command
```

## developer shell
using nix _and_ having flakes feature enabled
```sh
nix develop
```

using default nix
```sh
nix develop --option experimental-features flakes --extra-experimental-features nix-command
```
(Note: direnv)