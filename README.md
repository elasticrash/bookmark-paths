# bmk (bookmark-path manager)

bmk is a cli tool for managing directory paths. It allows you to bookmark frequently used paths, remove them when no longer needed, and quickly navigate to your saved locations. If you feel adventurous you can commit the config as a dot file.

This is a personal tool. You are free to use and that's it.

## Prerequisites

Since I don't provide binaries, you need to have Rust installed in your system in order to build this tool.

## Building
### Manually
* Build the project `cargo build --release`
* Move the binary to usr/local/bin
### Cargo

well just do `cargo install bmk`

## Usage
* `bmk add` adds the current path to the list of bookmarks
* `bmk list` lists the saved paths
* `bmk remove` removes the current path from the list
* `cd $(bmk go 1)` goes to the first index
* paths are stored in a dot file `~/.config/.bookmark-paths`

Just a brief explanation why you still need to use the cd command. The obvious part is that what `bmk go 1` actually does is it prints the path in the console. the `$()` captures the stdout of the execution into a local variable, which then allows cd to act upon.

`cd` is a part of the shell's core functionality, which means that when you execute cd within a process, it affects the current directory of that specific process. Once you exit the process, your working directory reverts to the state of the shell where you originally started.

:) enjoy
