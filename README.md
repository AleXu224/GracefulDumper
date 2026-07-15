# GracefulDumper

## What
This is a fork of the original [GracefulDumper](https://github.com/thexeondev/GracefulDumper), a Zenless Zone Zero dumper, updated to version 3.0. Made for educational and recreational purposes.
I have updated the dumper to work with the latest version of the game, though I learned how to reverse engineer specifically for this project so not all might be perfect.

## Features
This software can help you with dumping:
- C# definitions
- Protocol Buffers definitions
##### Bonus: `script.json` for applying dumped names in IDA

## Implementation
This dumper is written in Rust, without use of any 3rd party dependencies (except `windows` crate for WINAPI). It also implements idiomatic rust bindings for internal il2cpp functions.

## Usage
Just compile it in release mode (`cargo build --release`), put the launcher.exe in client folder and run it. Launcher will inject itself and dump all the definitions after il2cpp initializes. Supported game client: `OSCBWin3.0`.

## Contributing
Contributions are welcome. You can submit a pull request or open an issue if you find a bug
