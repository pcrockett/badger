# badger

Dead simple notifications without a daemon, in your terminal.

Output of `badger help`:

```
Publish and view notifications in your terminal

Usage: badger <COMMAND>

Commands:
  publish  Publish a notification
  run      Run a command and publish a notification if it fails
  next     Display the next notification in the list
  count    Get notification count
  pending  Determine by exit code if notifications are pending
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Currently only the `x86_64-unknown-linux-musl` platform triple is supported. PRs adding
support for other platforms are welcome.

## starship

Display a notification badge in your terminal prompt via [starship](https://starship.rs/):

```toml
[custom.badger]
when = 'badger pending'
symbol = '󱅫'             # nerd font: <https://www.nerdfonts.com/>
style = 'fg:226 bg:124'  # ansi colors: red background, yellow foreground
format = '[ $symbol ]($style) '
shell = ['sh']
```

_On my old laptop from 2008 this adds about 4ms to the prompt time._

## install

1. Download the executable from [releases](https://github.com/pcrockett/badger/releases)
2. `mv badger-x86_64-unknown-linux-musl badger && chmod +x badger`
3. Put the executable somewhere in your `$PATH`

## how it works

Badger publishes notifications simply by placing JSON files in `~/.local/state/badger`
(by default). Reading and removing a notification is as simple as reading the oldest
file in that folder, then deleting it.

Dependency-free, no D-Bus required. Works well on headless machines, works fine via SSH.
Simple. Naive. Good enough.
