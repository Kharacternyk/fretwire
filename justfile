default: lint test

lint:
    cargo clippy --all-targets

test:
    cargo nextest run

fix:
    cargo clippy --all-targets --fix --allow-staged
    cargo fmt
