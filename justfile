[private]
_default:
    @just --list

# Run cargo build
build profile="dev":
    cargo build --profile {{profile}}

# Run pre-commit on all files
lint:
    pre-commit run --all --show-diff-on-failure --color always

# Run bats tests
test: (build "release")
    bats ./tests

# Install badger to ~/.local/bin
install: (build "release")
    install target/release/badger ~/.local/bin

# Start release GitHub workflow
release:
    gh workflow run release.yml
