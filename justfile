format:
    cargo fmt
    cargo clippy

dep-graph:
    cargo modules dependencies \
        --no-sysroot \
        --no-fns \
        --no-types \
        --no-private \
        --no-traits \
        --layout dot \
        --lib \
        | sed 's/constraint=false/constraint=true/g' \
        | dot -Tpng > .ignored/mods.png

test:
    cargo test -- --test-threads=1