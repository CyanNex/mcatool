# mcatool

A simple command line tool that allows you to easily work with .mca (Minecraft region) files

### Usage:
```
mcatool <SUBCOMMAND>
```

### Usage:
```
extract    Extract a single chunk from a .mca file
trim       Trim all inactive chunks from a world
help       Print this message or the help of the given subcommand(s)
```

### Example:
To trim (remove) all chunks which have been inhabited for less than 5 minutes
in the `world` folder:
```
mcatool trim world 6000
```
