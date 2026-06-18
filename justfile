[private]
_default:
    @just --list

# Run pre-commit on all files
lint:
    pre-commit run --all --show-diff-on-failure --color always

# Run bats tests
test:
    bats ./tests

# Install badger to ~/.local/bin
install:
    install badger ~/.local/bin
