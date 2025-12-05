# Terebellum

Terebellum is a Rust crate intended as a playground for neural network research
and experimentation using the [Burn](https://burn.dev) framework with the
`ndarray` backend. The crate currently focuses on foundational utilities such as
a shared `Error` type, a lightweight `Result` alias, and ergonomic wrappers that
make it easier to prototype future models.

## Features

- **Ergonomic error handling:** A simple `Error` enum and `Result` alias keep
  error propagation consistent across the crate.
- **Newtype wrapper utilities:** The `W<T>` wrapper in the prelude demonstrates
  how to leverage `Deref`/`DerefMut` while maintaining strong typing.
- **Burn ecosystem ready:** Dependencies include `burn` and `burn-ndarray`,
  preparing the crate for training and inference experiments as additional
  modules are filled in.

## Getting started

1. Ensure you have a recent Rust toolchain (the repository pins the toolchain in
   `rust-toolchain.toml`).
2. Clone the repository and run the test suite:

   ```bash
   cargo test
   ```

As the project evolves, you can add your own layers, models, and utilities under
`src/nn`, `src/model`, and `src/utils` respectively.

## License

This project is licensed under the GNU General Public License version 3 or (at
your option) any later version. See [LICENSE](LICENSE) for the full text.
