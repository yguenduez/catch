mod docker 'docker.just'
mod hurl 'tests/hurl.just'

run:
    cargo run

alias v := verify

verify:
    cargo fmt -- --check
    cargo check
    cargo clippy
    cargo test
    just docker run
    just hurl test
    just docker stop

fmt:
    cargo fmt
    cargo clippy --fix --allow-dirty
