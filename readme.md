# darksplit - a command line speedrun timer

`darksplit` is a lightweight and fast speedrun timer based on livesplit-core
that runs in your terminal. With about 600 lines of C you are able to have a
suprisingly usable timer that can run on a lot of hardware.

Code quality is a bit poor at the moment but a code refactor is planned (Will 
probably never happen)

### Usage

```
darksplit <split file> [layout file]
```

## Building

You will need the following dependencies:

* `libjansson >= 2.0`
* `libncursesw`
* `make`
* latest stable `rustc`
* any c compiler with c99 support.

Get the submodules, edit the Makefile, then run `make`

# Configuration:

You are intended to configure darksplit by modifying its code.
It is pointless to make binary packages of it for this reason.

Hotkeys are in `./src/darksplit.c`

Colors are in `./src/color.c` (use ncurses color pairs)

# To Do

- Refactor the code.
