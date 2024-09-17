[![build](https://github.com/us-irs/va416xx-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/us-irs/va416xx-rs/actions/workflows/ci.yml)

Vorago VA416xx Rust Support
=========

This crate collection provides support to write Rust applications for the VA416XX family
of devices.

## List of crates

This workspace contains the following crates:

- The [`va416xx`](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/va416xx)
  PAC crate containing basic low-level register definition
- The [`va416xx-hal`](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/va416xx-hal)
  HAL crate containing higher-level abstractions on top of the PAC register crate.
- The [`vorago-peb1`](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/vorago-peb1)
  BSP crate containing support for the PEB1 development board.

It also contains the following helper crates:

- The [`bootloader`](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/bootloader)
  crate contains a sample bootloader strongly based on the one provided by Vorago.
- The [`flashloader`](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/flashloader)
  crate contains a sample flashloader which is able to update the redundant images in the NVM which
  is compatible to the provided bootloader as well.
- The `examples` folder contains various example applications crates for the HAL and the PAC.
  This folder also contains dedicated example applications using the
  [`RTIC`](https://rtic.rs/2/book/en/) and [`embassy`](https://github.com/embassy-rs/embassy)
  native Rust RTOSes.

## Using the `.cargo/config.toml` file

Use the following command to have a starting `config.toml` file

```sh
cp .cargo/def-config.toml .cargo/config.toml
```

You then can adapt the `config.toml` to your needs. For example, you can configure runners
to conveniently flash with `cargo run`.

## Using the sample VS Code files

Use the following command to have a starting configuration for VS Code:

```sh
cp -rT vscode .vscode
```

You can then adapt the files in `.vscode` to your needs.

## Flashing, running and debugging the software

You can use CLI or VS Code for flashing, running and debugging. In any case, take
care of installing the pre-requisites first.

### Pre-Requisites

1. [SEGGER J-Link tools](https://www.segger.com/downloads/jlink/) installed
2. [gdb-multiarch](https://packages.debian.org/sid/gdb-multiarch) or similar
   cross-architecture debugger installed. All commands here assume `gdb-multiarch`.

### Using CLI

You can build the blinky example application with the following command

```sh
cargo build --example blinky
```

Start the GDB server first. The server needs to be started with a certain configuration and with
a JLink script to disable ROM protection.
For example, on Debian based system the following command can be used to do this (this command
is also run when running the `jlink-gdb.sh` script)

```sh
JLinkGDBServer -select USB -device Cortex-M4 -endian little -if SWD -speed 2000 \
  -LocalhostOnly -vd -jlinkscriptfile ./jlink/JLinkSettings.JLinkScript
```

After this, you can flash and debug the application with the following command

```sh
gdb-mutliarch -q -x jlink/jlink.gdb target/thumbv7em-none-eabihf/debug/examples/blinky
```

Please note that you can automate all steps except starting the GDB server by using a cargo
runner configuration, for example with the following lines in your `.cargo/config.toml` file:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "gdb-multiarch -q -x jlink/jlink.gdb"
```

After that, you can simply use `cargo run --example blinky` to flash the blinky
example.

### Using VS Code

Assuming a working debug connection to your VA108xx board, you can debug using VS Code with
the [`Cortex-Debug` plugin](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug). Please make sure that
[`objdump-multiarch` and `nm-multiarch`](https://forums.raspberrypi.com/viewtopic.php?t=333146)
are installed as well.

Some sample configuration files for VS code were provided and can be used by running
`cp -rT vscode .vscode` like specified above. After that, you can use `Run and Debug`
to automatically rebuild and flash your application.

If you would like to use a custom GDB application, you can specify the gdb binary in the following
configuration variables in your `settings.json`:

- `"cortex-debug.gdbPath"`
- `"cortex-debug.gdbPath.linux"`
- `"cortex-debug.gdbPath.windows"`
- `"cortex-debug.gdbPath.osx"`

The provided VS Code configurations also provide an integrated RTT logger, which you can access
via the terminal at `RTT Ch:0 console`. In order for the RTT block address detection to
work properly, `objdump-multiarch` and `nm-multiarch` need to be installed.
