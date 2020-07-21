# helloiup

An example which shows the experimental and incomplete dynamic Rust bindings
to the [IUP](https://www.tecgraf.puc-rio.br/iup/) library in use, including
callbacks.

To try the example, download the
[rsiup](https://github.com/mark-summerfield/rsiup) bindings and this example
and put them side-by-side in parallel directories, e.g., `parent/iup/` for
the `rsiup` bindings, and `parent/helloiup`. Then `cd` into `helloiup` and
do `cargo run --release`. This will fail the first time due to missing
libraries. Copy (or on Unix soft-link) the `iup/iup` directory to
`helloiup/target/release/` and this time it should build and run. Or, if you
have Python 3 installed, use the `run.py` script. (If it doesn't build make
sure you changed the `rsiup` directory to `iup` _or_ fix the `path` used in
`helloiup`'s `Cargo.toml` file. Note also that the provided `.so`s and
`.dll`s are for 64-bit systems.)

## License

helloiup is free open source software (FOSS) licensed under the Apache-2.0
license: see LICENSE.
