# darksplit - a command line speedrun timer

`darksplit` is a lightweight and fast speedrun timer based on livesplit-core
that runs in your terminal. With under 1000 lines of C, you have an easily
hackable timer that can probably run on PCs from 2010. There is no built in
layout and split editor, but [LiveSplit One](https://one.livesplit.org) can
be used for that purpose.

Code quality is a bit poor at the moment but a code refactor is planned after
global hotkeys are implemented.

### Usage

```
darksplit <split file> [layout file]
```

## License 

Everything under ./src/ (excluding ./src/LICENSE) is licensed under the GPL 2.0 

## Building

You will need the following dependencies:

* `libjansson >= 2.0`
* `libncursesw`
* `make`
* `rustc >= 1.37.0`
* any c compiler with c99 support.

Get the submodules, edit the Makefile, then run `make`

# Configuration:

Hotkeys are in `./src/darksplit.c`

Colors are in `./src/color.c`

# To Do

- Implement global hotkeys
