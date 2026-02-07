


run:
    cargo run

alias v := verify

verify:
    cargo fmt -- --check
    cargo check
    cargo clippy
    cargo test

fmt:
    cargo fmt
    cargo clippy --fix --allow-dirty
