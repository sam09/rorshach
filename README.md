# Rorshach
A watchman for your directories. Rorshach allows you to watch your directories for changes and trigger certain commands when these events occur.

### Usage

* `./rorshach -f <dir> [-config <config_path>] [-t <seconds_to_wait_before_reload>]`

The default config file lies at `~/.rorshach.conf`


### Config File

The config file has rules of the following form

```
EVENT  PATTERN   ACTION
...     ...       ...

```

`EVENTS` can be `CREATE`, `DELETE` or `MODIFY`
`PATTERNS` are patterns to match the files in a directory. Example `*.cpp` matches all the C++ files in a directory.
`ACTIONS` are commands that can be executed when a `EVENT` occures

There are two environment variable available while executing an action, they are :- 
`{FULLPATH}` - Full path to the file,
`{BASEDIR}` - Path to the directory that rorshach is watching

### Examples

```
CREATE  *   echo " New file named ${FULLPATH} created"
```

The above will print a line of the form `New file named <file-name> created` every time a new file is created.


```
MODIFY  *.cpp   g++ ${FULLPATH} {BASEDIR}/test
```

Whenver a change is detected in a c++ file, `rorshach` will compile that file and create an executable named `test` in the base directory


### TODO
- [ ] Add more events to listen like `Rename`
- [ ] Support execution of a chain of commands for a single event
- [ ] Move Command Line passing to a different struct
- [ ] Add a threadpool to execute each task once an event is spawned
- [ ] Add Tests?
- [x] Move parse_rules to an enclosing struct
