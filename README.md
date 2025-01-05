# GracefulDumper

## What
A proof-of-concept dumper for Zenless Zone Zero 0.2.0. Made for educational and recreational purposes.

## Features
This software can help you with dumping:
- C# definitions
- Protocol Buffers definitions
##### Bonus: `script.json` for applying dumped names in IDA

## Implementation
This dumper is written in Rust, without use of any 3rd party dependencies (except `windows` crate for WINAPI). It also implements idiomatic rust bindings for internal il2cpp functions.

## Usage
Just compile it, put the launcher.exe in client folder and run it. Launcher will inject itself and dump all the definitions after il2cpp initializes. Supported game client: `OSCBWin0.2.0`.

## Need help?
Consider joining our [discord server](https://discord.gg/reversedrooms)

## Contributing
Contributions are welcome. You can submit them in form of a `.patch` file in our discord server. After review, it may be merged to upstream.
