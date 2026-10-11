set positional-arguments

default: sort lint test

all: sort fmt lint test

check: (sort "--check") (fmt "--check") (lint "-D" "warnings") test

sort *args:
    cargo sort --workspace "$@"

fmt *args:
    cargo fmt "$@"

lint *args:
    cargo clippy --all-targets -- "$@"

test *args:
    cargo nextest run --no-fail-fast "$@"

fix: && fmt
    cargo clippy --all-targets --fix --allow-staged
