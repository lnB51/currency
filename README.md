# Instruction

Project contains a `README.pdf` file with instructions that you can use to better understand the modules, code, settings and many other things related

# Building project dev version

## Dependencies

Rust:
- https://www.rust-lang.org/tools/install

Tauri:
- https://tauri.app/

NodeJS:
- https://nodejs.org/en/download/package-manager

Building (in root directory):
- `npm i`
- `cargo tauri dev`

# Building project release version

## Dependencies

Same as for dev

Building (in root directory):
- `npm i`
- `cargo tauri build`

This will create two installers msi/nsis which can be used to install app
