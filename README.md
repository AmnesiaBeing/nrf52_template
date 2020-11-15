# `nrf52_template`

> A template for building applications for ARM Cortex-M4 MCU NRF52.

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M4
  targets. Run:

``` console
$ rustup target add thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

1. Instantiate the template.

``` console
$ git clone https://github.com/AmnesiaBeing/nrf52_template
$ cd nrf52_template
```

2. Modified Project Name And Dependencies In `Cargo.toml`

4. Modified `.vscode/launch.json' to your debugger.

My nrf52 chip is nrf52832_qfaa, if yours is not the same, modified the dependency in `cargo.toml` and `.vscode/launch.json`.

3. Install Openocd

I'm using Archlinux And Jlink Debugger. Install Openocd from openocd-git(AUR) instead of openocd release ver 0.10.0.

Other Linux Release Version also.

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
It is suggested to use vscode.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
