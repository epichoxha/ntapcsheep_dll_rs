# Create DLL 🦀

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-windows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)

# Overview

The repository in question serves as an essential starting point for security specialists interested in creating dll using the Rust programming language.

# Usage

Compiling the DLL:
```sh
cargo build
```
The DLL will then be located in the target directory:
```sh
target/release/hello.dll
```

# Testing for more advance capabilities

This will run shellcode loading code in a thread, but not as a direct process, probably issue with rundll

## COM proxy dll

Got it to work easier than previously though

# TODO

Add mutex :)