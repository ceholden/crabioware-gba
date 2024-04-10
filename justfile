# TODO
# Checkout, https://github.com/agbrs/agb/blob/v0.15.0/justfile
export CARGO_TARGET_DIR := env_var_or_default('CARGO_TARGET_DIR', justfile_directory() + "/target")
CLIPPY_ARGUMENTS := "" # "-Dwarnings -Dclippy::all -Aclippy::empty-loop"

build: build-roms

build-roms:
    just _build-rom "crabioware" "crabioware"

build-debug:
    just _build-debug agb
build-release:
    just _build-release agb
clippy:
    just _all-crates _clippy

test:
    just _test-debug crabioware

test-release:
    just _test-release crabioware

doctest:
    (cd crabioware && cargo test --doc -Z doctest-xcompile)

clean:
    just _all-crates _clean

fmt:
    just _all-crates _fmt
fmt-check:
    just _all-crates _fmt-check

run-example example:
    just _build-example "{{example}}"
    mgba-qt "$CARGO_TARGET_DIR/thumbv4t-none-eabi/debug/examples/{{example}}"

run-example-release example:
    just _build-example-release "{{example}}"
    mgba-qt "$CARGO_TARGET_DIR/thumbv4t-none-eabi/release/examples/{{example}}"

run-game:
    (cd "crabioware" && cargo run --release)

run-game-debug:
    (cd "crabioware" && cargo run)


_build-rom folder name:
    #!/usr/bin/env bash
    set -euxo pipefail

    RELEASE_FOLDER="releases/"
    GAME_FOLDER="{{folder}}"
    INTERNAL_NAME="{{name}}"

    GAME_NAME="$(basename "$GAME_FOLDER")"

    TARGET_FOLDER="${CARGO_TARGET_DIR:-$GAME_FOLDER/target}"
    GBA_FILE="$TARGET_FOLDER/$GAME_NAME.gba"

    (cd "$GAME_FOLDER" \
        && cargo build --release --target thumbv4t-none-eabi \
        && cargo clippy --release --target thumbv4t-none-eabi -- {{CLIPPY_ARGUMENTS}} \
        && cargo fmt --all -- --check)

    just agb-gbafix \
        --title "${INTERNAL_NAME:0:12}" \
        --gamecode "${INTERNAL_NAME:0:4}" \
        --makercode GC \
        "$TARGET_FOLDER/thumbv4t-none-eabi/release/main" \
        -o "$GBA_FILE"

    mkdir -p $RELEASE_FOLDER
    cp -v "$GBA_FILE" "${RELEASE_FOLDER}/${GAME_NAME}_$(date '+%Y%m%dT%H%M%S').gba"

agb-gbafix *args:
    (agb-gbafix {{args}})

_all-crates target:
    # should use cargo workspace?
    for CARGO_PROJECT_FILE in crabioware*/Cargo.toml; do \
        PROJECT_DIR=$(dirname "$CARGO_PROJECT_FILE"); \
        just "{{target}}" "$PROJECT_DIR" || exit $?; \
    done

_build-debug crate:
    (cd "{{crate}}" && cargo build --examples --tests)
_build-release crate:
    (cd "{{crate}}" && cargo build --release --examples --tests)
_test-release crate:
    just _build-release {{crate}}
    (cd "{{crate}}" && cargo test --release)
_test-release-arm crate:
    (cd "{{crate}}" && cargo test --release --target=armv4t-none-eabi)
_test-debug crate:
    just _build-debug {{crate}}
    (cd "{{crate}}" && cargo test --lib)
_test-debug-arm crate:
    (cd "{{crate}}" && cargo test --target=armv4t-none-eabi)
_clippy crate:
    (cd "{{crate}}" && cargo clippy --examples --tests -- {{CLIPPY_ARGUMENTS}})
_clean crate:
    (cd "{{crate}}" && cargo clean)
_fmt crate:
    (cd "{{crate}}" && cargo fmt --all)
_fmt-check crate:
    (cd "{{crate}}" && cargo fmt --all -- --check)

_build-example example:
    (cd crabioware && cargo build "--example={{example}}")
_build-example-release example:
    (cd crabioware && cargo build "--example={{example}}" --release)
