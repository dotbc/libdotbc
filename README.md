# .bc file format utilities

Both a native library and a CLI tool are provided.

## Building

Building the library and CLI tool requires the following:

* The [Rust](https://rust-lang.org) compiler, version 1.10 or greater.
* Make

After cloning this repository, run `cargo build` in the root directory.
This will compile the CLI tool into the `target/debug/` directory.

## Usage

Here is how you can get started with the `dotbc` CLI tool.

#### Getting a list of supported commands

`dotbc -h`

**Output:**

```
dotbc

A utility providing access to the .bc file type.

Usage:
    dotbc <cmd> [<args>...]
    dotbc (-h | --help)
    dotbc --version

Options:
    -h, --help      Show this message
    --version       Print the version

The most commonly used commands are:
    ls-files         List all files in a .bc archive
    read-file        Read a file contained by a .bc archive
    show-metadata    Show the .bc metadata as JSON
    write-file       Write a file to a .bc archive
```

You can then run `dotbc <cmd> -h` for usage documentation for each
command.

#### Creating a new .bc archive

`dotbc write-file -cf hello-world.bc hello.txt 'hello world'`

This creates a new .bc archive, hello-world.bc. It writes the string
'hello world' at `hello.txt` inside the archive.

#### Listing the contents of a .bc archive

`dotbc ls-files -f hello-world.bc`

**Output:**

```
hello.txt
```

#### Reading a file contained in a .bc archive

`dotbc read-file -f hello-world.bc hello.txt`

**Output:**

```
hello world
```

## License

libdotbc and the dotbc CLI tool are distributed under the terms of the
MIT license. See LICENSE for details.
