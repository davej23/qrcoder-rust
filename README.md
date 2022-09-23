# qrcoder -- A QR code reader written in Rust

## Features
- Can be used to read QR code(s) on screen (by taking a screen capture), or from an image file.
- Copies decoded QR code string to clipboard and prints to stdout.

## Build Application
- Application needs to be compiled by running `cargo build --release` inside `qrcoder` folder.
- Executable file can be found in `qrcoder\target\release` folder.
- Please note that this file is portable and can be moved wherever necessary.

## Usage (Mac/Linux)
`./qrcoder` -- Extract QR code(s) from current screen.
`./qrcoder --image FILENAME` -- Extract QR code(s) from image file.

## Usage (Windows)
`.\qrcoder.exe` -- Extract QR code(s) from current screen.
`.\qrcoder.exe --image FILENAME` -- Extract QR code(s) from image file.
