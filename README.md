# Zonkey

Zonkey is a new programming language that I'm creating for my final year project. The design will focus on allowing adults and children inexperienced in software development to very easily develop GUI applications which they can share with people across the world. An interpreter and a browser will be created to run Zonkey applications.

## Programs

### Zonkey CLI Interpreter

Location: "zonkey_source/zonkey"

This program is a CLI interface to the Zonkey interpreter.

A Zonkey source file can be run by providing a path to that file as an argument:
```sh
$ zonkey test.zonk
```

### Zonkey Browser

Location: "zonkey_source/zonkey-browser"

This program loads a browser GUI that lets users easily run Zonkey programs over the internet or from their local file system.

Once installed to the system, this GUI can be launched with the following command:
```sh
$ zonkey-browser
```

## Compiling and Running Programs

A functioning installation of a Rust toolchain is required to compile executables. Instructions on how to set this up can be found at <https://www.rust-lang.org/tools/install>.

Navigate to the "zonkey_source" directory. Executables in this workspace can be compiled and run without installation by using the following command:
```sh
$ cargo run -p <PACKAGE NAME>
```

For example, if you wanted to run the "zonkey" CLI interpreter:
```sh
$ cargo run -p zonkey
```

Or if you wanted to run the "zonkey-browser":
```sh
$ cargo run -p zonkey-browser
```

When the program is invocated like this, additional argumemnts can be passed with the following syntax:
```sh
$ cargo run -p zonkey -- test.zonk
```

When invocating "cargo run" without the argument "-r" (for a release build), a debug build is created. This debug build reports a lot of debugging information to stdout, which can vastly reduce performance and can make it hard to use the CLI interface to zonkey. If you want the fast compiliation of debug builds, but not the debug information, use the following:
```sh
$ cargo run -p zonkey --profile no-debug-info -- test.zonk
```

You can also install the release builds of executables to your system for easier use. Navigate to the directory of the package and then use 'cargo install --path=. --force' to add this binary to your path. You can install the "zonkey" CLI interpreter this way by doing the following:
```sh
$ cd zonkey
$ cargo install --path=. --force
```

And then you can use it like a normal CLI utility:
```sh
$ zonkey test.zonk
```

## Documentation

### Building

To build the documentation, an installation of [hugo](https://gohugo.io/installation/) is required.

Once this is installed, navigate into the documentation directory.

To preview the documentation on the systems localhost, run:
```sh
$ hugo serve
```

To build the website, run the following command, which will output the generated code in the public directory:
```sh
$ hugo
```
