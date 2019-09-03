# darksplit - a command line speedrun timer

`darksplit` is a timer I put together in under 24 hours using livesplit-core. 
Currently it only runs on Linux and is missing a few features I want to 
implement but it is currently usable. Note that the code might be a bit messy
but I did attempt to keep it somewhat clean.

# License 

Everything under ./src/ (excluding ./src/LICENSE) is licensed under the GPL 2.0 

# Building

You will need the following dependencies:

* `libjansson >= 2.0`
* any ncurses version with wide character support
* gnu make
* a working rust compiler
* a working c compiler

Once you have the dependencies building is as easy as doing the following:

```
# get the submodules
git submodule update --init --recursive
# if all goes well the following should result in everything being built
make
```

# Configuration:

Hotkeys are in `./src/darkstatus.c`

Colors are in `./src/color.c`

The width for the layout is in `./src/darkstatus.h`


# To Do

- Implement a basic detailed timer
- Implement dynamic resizing
- Implement global hotkeys

# Will not implement

- The graph component (obvious reasons)
- Double height mode for all of the components
- Icons (obvious reasons)
- Split editor (use one.livesplit.org)
- Layout editor (use one.livesplit.org)
