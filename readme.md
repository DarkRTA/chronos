# darksplit - a command line speedrun timer

`darksplit` is a lightweight and fast speedrun timer using livesplit-core
that runs in your terminal. With about 600 lines of C you are able to have a
suprisingly usable timer that is also extremely lightweight.

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

You are intended to configure darksplit by modifying its code. It is pointless
to make binary packages of it for this reason. Nevertheless, most of the stuff 
that you'd want to modify is in `config.c`.
