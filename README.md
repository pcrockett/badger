# badger

dead simple notifications without a daemon, in your terminal

simply manipulates files in `~/.local/state/badger`:

- `badger publish` adds a file to the directory with contents you specify
- `badger next` displays the oldest file in the directory and deletes it
- `badger pending` makes it easy to use in terminal prompt functions

currently only the `x86_64-unknown-linux-gnu` platform triple is supported. PRs adding
support for other platforms are welcome.

## starship

display a notification badge via [starship](https://starship.rs/):

```toml
[custom.badger]
when = 'badger pending'
symbol = '󱅫'             # nerd font: <https://www.nerdfonts.com/>
style = 'fg:226 bg:124'  # ansi colors: red background, yellow foreground
format = '[ $symbol ]($style) '
shell = ['sh']
```

_on my old laptop from 2008 this adds about 4ms to the prompt time._

## install

1. download the executable from [releases](https://github.com/pcrockett/badger/releases)
2. `mv badger-x86_64-unknown-linux-gnu badger && chmod +x badger`
3. put the executable somewhere in your `$PATH`
