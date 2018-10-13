# peke_envs (peek poke environment variables)

[![asciicast](./docs/asciinema.png)](https://asciinema.org/a/206309)

## Concepts

* **Experimental purpose**
* Peek(Read) **current** environment variables in a arbitrary process.
* [WIP] Poke(Overwrite) environment variables in a arbitrary process.

*You can do the same things easy/safely by using gdb.*

## How it works

* Specify a target process as the tracee by pid.
* Search location for libc and __environ of the tracee.
* Exec Ptrace Attach to the tracee.
* By executing Ptrace Peek, read and follow pointers, and read environment variables.
* [WIP] By executing Ptrace Peek, overwrite environment variables.

## Usage

* Peek environment variables.
```
$ peek <pid>
```

* [WIP] Poke environment variables.
```
$ peek <pid> <key> <value>
```

## License

[MIT](./LICENSE)

## Author

[Satoshi Tajima](https://github.com/s-tajima)

