# Computes VM

This is the VM part for my WIP-Mod [Computes](https://github.com/feldim2425/Computes).

# What it should do

The VM handles the actual WASM mechanism (using [Wasmer](https://wasmer.io/)) including state saving, state loading, module loading, cross-module communication, communication to the VM (possibly WASI), time & memory limits aWnd actually executing the modules.

# What it actually does

Currently nothing. Still WIP.

# Architecture
(not final)

The project defines a Wasmer middleware to handle time limits, as well as a tunable to limit memory usage across modules. Multiple modules can be loaded into one VM instance, if enabled. 

A integrated "firmware" / "bios" will handle most of the APIs exposed to the software running on the computers. The computer has to provide a entry module to handle the startup and loading of other modules (if allowed), and/or handle the user interface as well as peripherals in-game.

The VM will expose all necessary APIs to the JVM, like creating/destroying VMs, saving/loading states and modules, adding APIs and set limits (based on in-game conditions like items used).


[Wasmer Architecture](https://docs.rs/wasmer/1.0.1/wasmer/#project-layout)

# Rust ![Rust](https://www.rust-lang.org/logos/rust-logo-32x32.png)

This project is mainly written in [Rust](https://www.rust-lang.org/) (not the game; the programming language).

There are many reasons why:
* I like Rust and want to learn it a bit better.
* Wasmer (the WASM Runtime) is written in Rust, so Rust is the simplest way to interface with Wasmer.
* Safe(r) than using C/C++ especially since it should handle code written by random people uploaded to some random server.
* (again) I like Rust. 😊

