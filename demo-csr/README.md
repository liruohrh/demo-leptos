# prepare

- env
  - `cargo install trunk`
  - `rustup target add wasm32-unknown-unknown`
- project
  - `cargo init demo-csr`
  - `cargo add leptos --features=csr`
- run
  - `trunk serve --open`

# fromat

- rustfmt.toml
- rust-analyzer.toml
- leptosfmt.toml
