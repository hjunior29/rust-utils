# rust-utils

A small, tested Rust utility library. Each utility lives in its own source file and has automated tests.

## Requirements

Rust 1.85 or newer and Cargo. No external dependencies are needed.

## Build and test

```sh
cargo fmt --all
cargo clippy --all-targets --locked --offline -- -D warnings
cargo build --locked --offline
cargo test --locked --offline
```

Install formatter and lint components with `rustup component add rustfmt clippy`. CI requires clean formatting and rejects Clippy warnings.

## Usage

```rust
use rust_utils::{reverse_string, word_count, clamp_int};

assert_eq!(reverse_string("hello"), "olleh");
assert_eq!(word_count("hello world"), 2);
assert_eq!(clamp_int(12, 0, 10), Ok(10));
```

## Utilities

- `reverse_string` reverses Unicode code points.
- `word_count` counts whitespace-separated words.
- `clamp_int` clamps an integer to inclusive bounds and rejects reversed bounds.

- `common_prefix` returns the longest shared prefix by Unicode code points.

- `normalize_whitespace` trims Unicode whitespace and collapses runs to one ASCII space.

## Adding utilities

Use English for code, comments, documentation, and tests. Add one utility per file with meaningful tests covering normal, empty, boundary, and invalid input where applicable. Do not add dependencies without review.

Run the complete build and test suite before submitting changes. Keep public exports synchronized when adding modules.

## License

MIT
