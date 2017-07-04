# Trebuchet
## a productivity Launcher written in Rust

This is currently a WIP and will hopefully be useable some time in the 
near future. 

So far only running command and the Instant Answer query to DuckDuckGo
are working (the DDG query will only print to stdout if there are 
definitive unique results).

### Build and dependencies

All you need to build is cargo (and its dependencies) and then simply run `cargo build` to build
or `cargo run` to run the project.

### Usage

After launching the program you are prompted to input your command.
Commands can be prefixed by the following terms so far:
* `run` to run a command.
* `quack` to query a DuckDuckGo Instant Answer