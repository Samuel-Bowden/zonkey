# Zonkey

Zonkey is a new programming language that I'm creating for my final year project. The design will focus on allowing adults and children inexperienced in software development to very easily develop GUI applications which they can share with people across the world. An interpreter and a browser will be created to run Zonkey applications.

## Programs

### Zonkey CLI Interpreter

Location: "zonkey_source/zonkey"

This program is a CLI interface to the Zonkey interpreter. To get detailed help for this tool, run:
```sh
zonkey --help
```

This program has two modes, one for running source files and another for providing a prompt.

The prompt can be accessed by just running:
```sh
$ zonkey
```

A Zonkey source file can be run by providing a path to that file as an argument:
```sh
$ zonkey test.zonk
```

Debug information can be seen in both modes by adding the following argument:
```sh
$ zonkey test.zonk -d
```

### Zonkey Browser

Location: "zonkey_source/browser"

This program is not implemented yet, development on this will start later in the project.

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

When the program is invocated like this, additional argumemnts can be passed with the following syntax:
```sh
$ cargo run -p zonkey -- test.zonk -d
```

You can also install the executables to your system for easier use and faster performance. You can install the "zonkey" CLI interpreter by doing the following:
```sh
$ cd zonkey
$ cargo install --path=. --force
```

And then you can use it like a normal CLI utility:
```sh
$ zonkey test.zonk -d
```

## Documentation

Up to date documentation for Zonkey can be viewed at <https://zonkey.sambowden.rocks>.

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
