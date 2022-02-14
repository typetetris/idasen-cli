![Build](https://github.com/typetetris/idasen-cli/workflows/Build/badge.svg)

idasen-cli
==========

Control your IKEA IDÅSEN standing desk by a cli tool using Bluetooth.

(Still some work in progress.)


Basic Usage
-----------
    CLI to Control IKEA IDÅSEN standing desk via Bluetooth.
    
    USAGE:
        idasen-cli <SUBCOMMAND>
    
    OPTIONS:
        -h, --help       Print help information
        -V, --version    Print version information
    
    SUBCOMMANDS:
        debug      Dump data about desks we can find
        help       Print this message or the help of the given subcommand(s)
        list       List all saved positions
        listen     Listen to position and speed changes
        pos        Moves desk to position <pos> mm
        rel        Moves desk <rel> mm. Positive numbers move up, negative numbers move down
        restore    Move desk to position saved under a certain name
        save       Save position of desk under a certain name
        show       Show the current position

Further Ideas
-------------

1. Remove `debug` and `listen` maybe.
1. Improve `move_to`, if possible.
