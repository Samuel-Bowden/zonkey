# Zonkey

Welcome to my final year computer science project, Zonkey. For this project, a novel scripting language was created, which is used to write graphical applications for a browser developed with the [Iced GUI toolkit](https://github.com/iced-rs/iced). The aim of this project is to explore a simplier way for novice developers to create their first graphical applications, which can then be loaded by friends and family over the internet. It aims to be easier to learn compared to existing web technologies. Furthermore, applications written for Zonkey can optionally be installed to the system (using the Zonkey runtime in the background), allowing offline usage and tighter integration with the desktop environment.

<div align="center">
    <p><b>Fig 1. Zonkey browser running a "Binary Bulletin" news application loaded over the network.</b></p>
</div>

![screenshot](https://github.com/Samuel-Bowden/zonkey/assets/91887909/7e43bd6f-7709-4096-996a-bd18e92a8b4b)


<div align="center">
    <p><b>Fig 2. Developing the calculator app, with a portion of the Zonkey script displayed on the right.</b></p>
</div>

![screenshot](https://github.com/Samuel-Bowden/zonkey/assets/91887909/b1d73311-81b2-41a2-b763-68752dd04cd4)

<div align="center">
    <p><b>Fig 3. Calculator app installed on an Ubuntu system, running as an individual app.</b></p>
</div>

![screenshot](https://github.com/Samuel-Bowden/zonkey/assets/91887909/86782567-eeb8-4334-ba62-2806a2370c79)


## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
    - [Windows](#windows)
    - [Linux](#linux)
- [Usage](#usage)
    - [First Taste Of The Language](#first-taste-of-the-language)
    - [The Browser](#the-browser)
- [Documentation and Learning Material](#documentation-and-learning-material)
- [Compilation](#compilation)
- [Testing](#testing)
    - [Unit and Integration Testing](#unit-and-integration-testing)
    - [Fuzz Testing](#fuzz-testing)
    - [News Server](#news-server)
    - [Benchmarking](#benchmarking)

## Requirements

| Requirement | Description |
| ------ | ------ |
| Operating System | **Windows** (10 or later), **Linux** (Tested on Ubuntu 22.04, Fedora Workstation 37) |
| GPU | **Any that support OpenGL 3.0+ or OpenGL ES 2.0**.  Most modern systems support these, so compatibility should not be a major concern. However, it's important to ensure that high-quality graphics drivers are installed, particularly if you're running Zonkey on a Windows VM. Some Windows VMs may not come with OpenGL graphics drivers preinstalled, and without support for these APIs, the Zonkey GUI will be unable to launch.

## Installation

Zonkey is pre-compiled for supported operating systems, and can be installed following the instructions for each operating system below.

### Windows

Please download the file named "Zonkey-1.0.2-x86_64.msi" inside the releases. Double click on the downloaded MSI file, and the installer will guide you through the process of adding Zonkey to your system.

After installation, "zonkey" will be added to your path and a shortcut and desktop entry to the browser will be created.

To uninstall, either launch this MSI installer again and select uninstall, or go to add or remove programs in Windows settings.

You can now move onto the [usage](#usage) section.

### Linux

For Linux users, download the `linux-installer.zip` file inside the releases section.

Then navigate into your downloads, decompress the folder and run the install script.

```sh
$ cd Downloads
$ unzip linux-installer.zip
$ cd linux-installer/
$ bash install.sh
```

Zonkey should now be added to your path, which can be verified using the command below. You should now also see a desktop icon for Zonkey in your start menu.

```sh
$ zonkey --version
```

To uninstall at a later date, navigate to this directory again and run the uninstall script.

```sh
$ bash uninstall.sh
```

You can now move onto the [usage](#usage) section.

## Usage

Zonkey provides a command line interface to develop scripts, and a browser to find, use and share applications.

### First Taste Of The Language

After installation, you should be able to run the following command to receive an overview of the available commands in Zonkey. You can open this in the terminal emulator of your operating system, e.g. Powershell on Windows.

```sh
$ zonkey
The Zonkey Programming Language And Browser.

Usage: zonkey <COMMAND>

Commands:
  run      Run the script in the command line interface, opening a window if a page is set
  browser  Run the browser
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

You can see the help of subcommands like so:
```sh
$ zonkey run --help
# And
$ zonkey browser --help
```

To get your first taste of the Zonkey programming language, launch your favorite code editor such as VSCode and enter the following. 

```zonk
start {
    println("Hello World!");
}
```
Save the file as `hello_world.zonk`.

Open a terminal where the file is located and run the following command:
```sh
$ zonkey run hello_world.zonk
Hello World!
```

Lets try something slightly more fancy to see how GUIs are created with the Zonkey programming language.

Open another file in your code editor and enter the following.

```zonk
start {
    let blue = "#0000AA";

    let message = Text("Hello GUI!")
        .set_colour(blue)
        .set_size(50.0);

    let my_first_page = Page()
        .add(message)
        .center();

    set_page(my_first_page);
}
```
Save this file as `hello_gui.zonk` and run like so.

```sh
$ zonkey run hello_gui.zonk
```

Please visit the documentation in the browser, as discussed in the [documentation section](#documentation-and-learning-material) to learn more about the Zonkey programming language.

### The Browser

With Zonkey, you have the ability to easily browse and access other applications that have been created using the language. This includes built in applications, those available over the internet and even those on your own filesystem. By installing Zonkey, you should have automatically received a shortcut on your desktop or start menu that grants you access to this feature.

If you want, you can launch the browser on the command line like so, which is useful for testing your own applications.

```sh
$ zonkey browser
```
To launch your first GUI script that you developed before in the browser interface.
```sh
$ zonkey browser hello_gui.zonk
```

Zonkey applications found in the browser can offer installation to your system for offline use and a better integration with your desktop, which is demonstrated with the applications located at `zonkey:calculator/app.zonk` and `zonkey:guessing_game/app.zonk` - links to these can be found on the homepage of the browser.

## Documentation and Learning Material

Zonkey's documentation and learning material, written in Zonkey script itself, is easily accessible within the browser at `zonkey:documentation/index.zonk` - a link to this can be found on the homepage of the browser.

This covers how to use the language and guides on how to use the GUI API, and also documentation of the standard prelude.

<div align="center">
    <p><b>Fig 4. An example page inside the documentation.</b></p>
</div>

![screenshot](https://github.com/Samuel-Bowden/zonkey/assets/91887909/2d4b1431-3384-4909-9a7b-a30e0adee462)

The source code for the documentation and learning material is located under `interpreter/assets/documentation`.

## Compilation 

A functioning installation of a Rust toolchain is required to compile Zonkey. Instructions on how to set this up can be found at <https://www.rust-lang.org/tools/install>.

There should be no need for additional requirements as the necessary development headers are included with Windows, Ubuntu, and Fedora. However, if your compilation is failing, please read the failed compilation output as it will detail what the development headers it could not find, as this may happen on minimal distributions of Linux.

Once you have a functioning toolchain for Rust, first clone the repository:

```sh
$ git clone https://github.com/Samuel-Bowden/zonkey
```

Navigate into the directory and start the compilation.
```sh
$ cd zonkey
$ cargo build --release
```
Once this has completed, the binary will be located at `target/release/zonkey`. To install this to your system path, you can do:

```sh
$ cargo install --path .
$ zonkey --version
```

To run the program without installing to your path, you can do the following.
```sh
$ cargo run -r -- --version
# Or
$ ./target/release/zonkey --version
```

If you are interested in testing the debug mode of Zonkey, which prints the current state of the interpreter during execution, you can try using the debug build profile.

```sh
cargo build --profile debug-info
```

This will be located at `target/debug-info/zonkey` and can be run like so.

```sh
$ cargo run --profile debug-info
# Or
$ ./target/debug-info/zonkey
```

## Testing

### Unit and Integration Testing

For this, you can simply run the following the root of the repository. Please note that some test cases require an internet connection.
```sh
cargo test
```

### Fuzz Testing

Fuzz testing only works on Linux and requires certain parameters to be set in the kernel, however, the afl tool will guide you on what to do. If the fuzz testing is attempted, you first need to install the afl tester.

```sh
cargo install afl
```

Then compile the fuzzer.

```sh
cargo afl build -p tests/fuzzer --profile fuzzer
```
Finally, start the fuzzer, feeding it some scripts to seed with in one of the testing directories.
```sh
cargo afl fuzz -i interpreter/src/tests/scripts -o out target/fuzzer/fuzzer
```

### News Server

The news server requires Python and Flask to be installed to the system. Navigate to the `tests/example-server` directory and run the server:
```
$ cd tests/example-server
$ python server.py
```
Ensure that port 8000 is free. Then in the browser, you can either click the link on the homepage to `Binary Bulletin`, or type `http://localhost:8000` in the address bar to load the homepage. To clean the comments added, you can run `http://localhost:8000/clean`.

### Benchmarking

To run the benchmarks, the following compilers and interpreters need to be installed, along with some additional software. Note that benchmarking only currently works on Linux.

- Rustc (will be included with the Rust toolchain)
- Clang
- GCC
- CPython
- Bun
- Deno
- clox (has to be manually compiled from their [repository](https://github.com/munificent/craftinginterpreters) and added to path)
- Zonkey itself installed to the path.
- Perf
- Diffuse for verification

Once these are installed, navigate to the benchmarking directory, and run a benchmark using the testing script. Specify the test, the number of cycles and optionally add Lox at the end to include it in the benchmarking test if a script has been created for it. Results will be written inside the `tests/benchmarks/results` directory.
```sh
$ cd tests/benchmarks
$ bash tester.sh hello_world 100 lox
$ bash tester.sh fibonacci 10 lox
$ bash tester.sh leibniz 10
$ bash tester.sh objects_arrays 10
```
You can also verify that each program is creating the same output as so.

```sh
$ bash verifier.sh hello_world lox
$ bash verifier.sh fibonacci lox
$ bash verifier.sh leibniz lox
$ bash verifier.sh objects_arrays lox
```
