set dotenv-load
set shell := ["zsh", "-uc"]

now := `date +%s`
target := "debug"

aseprite_args := "--batch --format json-array --ignore-empty --list-tags --list-layers --split-layers --split-tags --trim --merge-duplicates --filename-format '{tag}#{frame}#{layer}'"

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

res-tex target:
  aseprite {{aseprite_args}} --sheet-type packed resources/src/{{target}}.aseprite --sheet resources/obj/{{target}}.png --data resources/obj/{{target}}.ase.json
  cargo xtask ase-to-res {{target}}
  rm resources/obj/{{target}}.ase.json

res-tm target:
  aseprite {{aseprite_args}} resources/src/{{target}}.aseprite --sheet resources/obj/{{target}}.png --data resources/obj/{{target}}.ase.json

run:
 RUST_BACKTRACE=1 cargo run --package binary

test:
  cargo nextest run
