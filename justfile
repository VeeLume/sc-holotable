# Your repo owns this file — the template never overwrites it (copier _skip_if_exists).
# Shared recipes live in common.just, kept in sync via `copier update`. `just --list` to see all.
import? 'common.just'

default:
    @just --list

# --- repo-local recipes below (add yours here) ---

# Only rustdoc reads this; it turns a broken intra-doc link into a build error.
export RUSTDOCFLAGS := "-D rustdoc::broken_intra_doc_links"

# API docs live in rustdoc (there is no docs/ dir): broken links and doctests
# that no longer compile are the staleness signal. The umbrella's module links
# only resolve with its features on, so it is checked separately.
doc-check:
    cargo doc --no-deps --keep-going --workspace --exclude sc-extract-generated --exclude sc-holotable
    cargo doc --no-deps -p sc-holotable --features full
    cargo test --doc --workspace
