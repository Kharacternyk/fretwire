default: sort lint test

sort:
    cargo sort --workspace

lint:
    cargo clippy --all-targets

test:
    cargo nextest run --no-fail-fast

fix:
    cargo clippy --all-targets --fix --allow-staged
    cargo fmt
