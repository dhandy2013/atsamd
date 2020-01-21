Embedded Apps for the Circuit Playground Express
================================================

This package contains a library crate with utility code for working with the
Circuit Playground Express, and multiple binary crates. Each binary create is an
embedded app that can be turned into a .uf2 file that can be loaded
as firmware for the [Adafruit Circuit Playground Express board](https://www.adafruit.com/product/3333).

Setup
-----

Set up the gcc toolchain and Rust to cross-compile for the Armv6m architecture
as described in the [atsamd repository README](https://github.com/atsamd-rs/atsamd/blob/master/README.md).

Get the [uf2conf.py script](https://github.com/Microsoft/uf2) and put it in your
PATH.

Building
--------

In this directory, run:
```
./build.py <app-name>
```
where ``<app-name>`` matches a source file named: ``src/bin/<app-name>.rs``

The build script will run ``cargo build``, then convert the output ELF file to a
``.bin`` file, then convert the ``.bin`` file to a deployable ``.uf2`` file.  If
a Circuit Playground Express is plugged in to the computer via USB, the build
script will also deploy the ``.uf2`` file to the board.
