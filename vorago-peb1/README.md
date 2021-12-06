# Rust BSP for the Vorago PEB1 development board

## Using the `.cargo/config.toml` file

Use the following command to have a starting `config.toml` file

```sh
cp .cargo/def-config.toml .cargo/config.toml
```

You then can adapt the `config.toml` to your needs. For example, you can configure runners
to conveniently flash with `cargo run`.

## Notes on board revisions

On RevA, issuing the `monitor reset` command in the GDB application is problematic and will prevent
the flashed binary from working properly. On board revision B, this was not an issue.
For that reason, two different `*.gdb` files were provided in the `jlink` folder for each
board revision. If you are not using these files, make sure to correctly configure your flash
tools depending on which tool or board your are using.
