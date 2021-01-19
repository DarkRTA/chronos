# chronos - a command line speedrun timer

`chronos` is a lightweight and fast speedrun timer using livesplit-core
that runs in your terminal. With about 600 lines of C you are able to have a
suprisingly usable timer that is also extremely lightweight.

### Usage

```
  Usage: ./chronos [options]

  Options:

    -V, --version                 output program version
    -h, --help                    output help information
    -c, --config <arg>            config file to use
    -l, --layout <arg>            layout file to use
    -s, --splits <arg>            split file to use
```

## Building

You will need the following dependencies:

* `make`
* latest stable `rustc`
* any c compiler with c99 support.

Get the submodules, edit the Makefile, then run `make`

# Licensing Info

`chronos` is licensed under the GNU GPL 2.0. See `./LICENSE` for the full
details.
