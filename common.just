# common.just — shared VeeLume recipes (template-owned; `copier update` keeps this in sync).
# Imported by the repo's justfile. Add repo-local recipes in justfile, not here.

test:
    cargo test --workspace

fmt:
    cargo fmt --all

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# What CI runs.
check:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings

clean:
    cargo clean
