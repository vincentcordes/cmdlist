# cmdlist

A cross platform cli program to help keep track of commands.


# Examples
- cmdlist -h
- cmdlist -q cargo
- cmdlist -q cargo -d true -g true

# Notes
- The crate colored, used to color output does not play nicely with Git Bash in Windows Terminal, or Git Bash in the VSCode terminal.
- If using Git Bash without the Windows 'wrapper' the output works as expected.
