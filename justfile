
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

test:
  cargo nextest run
