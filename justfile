default_target := "x86_64-unknown-linux-gnu"

[private]
_default:
    @just --list

# Run cargo build
build profile="dev" target=default_target:
    cargo build --profile {{profile}} --target {{target}}

# Run pre-commit on all files
lint:
    pre-commit run --all --show-diff-on-failure --color always

# Run bats tests
test target=default_target: (build "release" target)
    #!/usr/bin/env bash
    set -euo pipefail
    export BADGER_BUILD_TARGET="{{target}}"
    bats ./tests

# Install badger to ~/.local/bin
install target=default_target: (build "release" target)
    install "target/{{target}}/release/badger" ~/.local/bin

# Start release GitHub workflow
release:
    gh workflow run release.yml
