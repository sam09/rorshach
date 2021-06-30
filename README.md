
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
![Build](https://github.com/sam09/rorshach/workflows/Build/badge.svg)

# Rorshach ![rorschach.gif](http://www.gomotes.com/emoticon/rorschach.gif)

A watchman for your directories. Rorshach allows you to watch your directories for changes and trigger certain commands when these events occur.

### Installlation

At the moment the only way to install is from crates.io (https://crates.io/crates/rorshach).
Run `cargo install rorshach`  to install it on a linux machine.

### Usage

* `./rorshach -f <dir> [-config <config_path>] [-t <seconds_to_wait_before_reload>]`

The default config file lies at `~/.rorshach.conf`


### Config

Configuration file is a tab separated values (TSV) file, and has the following format:

```
EVENT  PATTERN   ACTION
...     ...       ...
```
#### Sample Configuration File
Please ensure you have physical TAB characters as delimiters. If you are using vim, you can use `:set list` to show invisible characters. It would look like this:

![image](https://user-images.githubusercontent.com/30809170/123977658-2fc52f00-d98d-11eb-9223-0f06b6d46bf1.png)

Otherwise, you will receive an error during start.

### Events

`EVENTS` can be `CREATE`, `DELETE` `RENAME` or `MODIFY`. Each event is trigger when a file in the directory being watched is CREATED, DELETED, MODIFIED or RENAMED respectively.
`PATTERNS` are patterns to match the files in a directory. Example `*.cpp` matches all the C++ files in a directory.
`ACTIONS` are commands that can be executed when a `EVENT` occures

There are following environment variable available while executing an action, they are :- 
`{FULLPATH}` - Full path to the file,
`{BASEDIR}` - Path to the directory that rorshach is watching
`{NEWFULLPATH}` - New Path to the file, when a file is renamed else empty.


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
- [x] Add more events to listen like `Rename`
- [ ] Support execution of a chain of commands for a single event
- [ ] Move Command Line passing to a different struct
- [ ] ~Add a threadpool to execute each task once an event is spawned~
- [x] Add a pub-sub mechanism for events
- [ ] Add Tests?
- [x] Move parse_rules to an enclosing struct
- [x] Use `log` create for logging.
- [x] Provide better messages for errors.
- [x] Add a pub sub mechanism to listening to events and consuming them
