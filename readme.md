# chronos

`chronos` is a lightweight speedrun timer that runs in your terminal.

## Building

You will need the following dependencies:

* latest stable rust

Run the following commands in your shell.

```
git clone --recursive https://github.com/darkrta/chronos
cd chronos
cargo build --release
```

The timer should be placed in `./target/`

## Usage

* See `./chronos --help` for command line args.
* Split files can be imported from both LiveSplit and Livepslit One.
* Layout files can only be used from LiveSplit One, and may require tweaking in
  order to look right. A best effort attempt is made to use colors from the
  layout as they are.

### Key Bindings
* Escape - Quit
* F1 - Activate Global Hotkeys
* F2 - Deactivate Global Hotkeys
* F3 - Save Splits (not done automatically)
* More key binds are in `./examples/config-example.toml`
