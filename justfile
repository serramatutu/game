
check: check-lint check-fmt 

check-lint:
  cargo clippy -- -Dwarnings

check-fmt:
  cargo fmt --check

fix: fix-lint fix-fmt

fix-lint:
  cargo clippy --fix --allow-dirty

fix-fmt:
  cargo fmt

build:
  cargo build --all

build-binary:
  cargo build --package binary

build-game:
  cargo build --package game

test:
  cargo nextest run
