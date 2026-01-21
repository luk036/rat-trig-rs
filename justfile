default:
    check
    test

check:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features --workspace

test:
    cargo test --all-features --workspace

build:
    cargo build --release

bench:
    cargo bench

doc:
    cargo doc --no-deps --document-private-items --all-features --workspace --examples

deny:
    cargo deny check

clean:
    cargo clean

ci: check test

.PHONY: default check test build bench doc deny clean ci
