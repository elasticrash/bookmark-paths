# bmk (bookmark-path manager)

bmk is a cli tool for managing directory paths. It allows you to bookmark frequently used paths, remove them when no longer needed, and quickly navigate to your saved locations. If you feel adventurous you can commit the config as a dot file.

This is a personal tool. You are free to use and that's it.

## Usage
* Build the project (cargo build --release)
* Move the binary to usr/local/bin
* `bmk add` adds the current path to the list
* `bmk remove` removes the current path from the list
* `bmk list` lists the saved paths
* `cd $(bmk go 1)` goes to the first index 
