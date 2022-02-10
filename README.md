![Continuous integration](https://github.com/typetetris/idasen-cli/workflows/Rust/badge.svg)
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
        debug       Show position of desk
        down        Start moving down
        help        Print this message or the help of the given subcommand(s)
        listen      Listen to position and speed changes
        position    Moves desk to position <pos> mm
        rel         Moves desk <rel> mm. Positive numbers move up, negative numbers move down
        show        Show the current position
        stop        Start moving down
        up          Start moving down

Ideas for next steps
--------------------

1. Make a command to store position named `<name>` in `~/.config/idasen-cli/<name>`.
1. Make a command to list stored positions.
1. Make a command to reposition to a stored position by name.

