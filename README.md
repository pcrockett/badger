# badger

dead simple notifications without a daemon, in your terminal

simply manipulates files in `~/.local/state/badger`:

- `badger publish` adds a file to the directory
- `badger next` displays the oldest file in the directory and deletes it
- `badger pending` makes it easy to use in terminal prompt functions

written in [nushell](https://www.nushell.sh/)

## starship

display a notification badge via [starship](https://starship.rs/):

```toml
[custom.badger]
when = 'badger pending'
symbol = '󱅫'             # nerd font: <https://www.nerdfonts.com/>
style = 'fg:226 bg:124'  # ansi colors: red background, yellow foreground
format = '[ $symbol ]($style) '
shell = ['nu']
```

_on my old laptop from 2008 this adds about 50ms to the prompt time._

## todo

this is currently a minimum viable product. if i have time:

- [ ] create interactive `fzf` interface
- [ ] add metadata to notifications
