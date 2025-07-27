set dotenv-load
set shell := ["zsh", "-uc"]

now := `date +%s`
target := "debug"

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
  @just _timestamp-game

build-binary:
  cargo build --package binary

build-game:
  cargo build --package game
  @just _timestamp-game

_timestamp-game:
  mv target/{{target}}/libgame.so target/{{target}}/libgame.so.{{now}}
  echo -n "{{now}}" > target/{{target}}/latest.txt

clean-libs:
  rm target/{{target}}/libgame.so.*

run:
 cargo run --package binary

test:
  cargo nextest run
