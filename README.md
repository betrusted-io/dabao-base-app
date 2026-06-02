# Dabao Base Application

This is a reference application for Dabao developers who want a starting
point for stand-alone applications.

This base application bundles in a REPL command shell that includes a few
demo commands. The base demonstration includes:

- `echo` test
- `test` commands for retrieving system information
- `ver` command that prints the version of Xous used for the build
- `i2cdetect` command that scans for any connected I2C devices
- `touch` which demonstrates BIO captouch capabilities
- `ws2812` which demonstrates the ability to drive a WS2812 LED strip with the BIO

Users looking to add new commands can follow the four-step guide inside `src/cmds.rs`, or
they can simply modify an existing command template by adding an entry to the
match statement (for example, see `src/cmds/test.rs`).

## Xous Toolchain

Xous is a Tier-3 Rust target, which means that you can't just run rustup update
to retrieve the `std` library for Xous. The `build.rs` script checks for the presence
of the Xoust `std` toolchain on every build, and ensures that it is installed.