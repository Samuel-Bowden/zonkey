# Zonkey

Zonkey is a new programming language and browser that I've created for my final year project. The design focuses on allowing adults and children inexperienced in software development to very easily develop GUI applications which they can share with people across the world.

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
    - [Windows](#windows)
    - [Linux](#linux)
- [Usage](#usage)
    - [First Taste Of The Language](#first-taste-of-the-language)
    - [The Browser](#the-browser)
- [Documentation and Learning Material](#documentation-and-learning-material)
- [Repository Organisation](#repository-organisation)
- [Compliation](#compilation)
- [Testing](#testing)
    - [Unit and Integration Testing](#unit-and-integration-testing)
    - [Fuzz-Testing](#fuzz-testing)
    - [News Server](#news-server)
- [Benchmarking](#benchmarking)
- [Third Party Code Used](#third-party-code-used)

## Requirements

| Requirement | Description |
| ------ | ------ |
| Operating System | **Windows** (10 or later), **Linux** (Tested on Ubuntu 22.04, Fedora Workstation 37) |
| CPU | **x86_64** for precompiled installations. Theoretically could work on other popular architectures such as ARMv8 by manual compilation, however, this is not tested. If you want to try, please visit the [compliation](#compilation) section.
| GPU | **Any that support OpenGL 3.0+ or OpenGL ES 2.0**.  Most modern systems support these, so compatibility should not be a major concern. However, it's important to ensure that high-quality graphics drivers are installed, particularly if you're running Zonkey on a Windows VM. Some Windows VMs may not come with OpenGL graphics drivers preinstalled, and without support for these APIs, the Zonkey GUI will be unable to launch.

## Installation

Zonkey is pre-compiled for supported operating systems, and can be installed following the instructions for each operating system below.

### Windows

Please download the file named "Zonkey-1.0.2-x86_64.msi" inside the installer directory of this repository. Double click on the downloaded MSI file, and the installer will guide you through the process of adding Zonkey to your system.

After installation, "zonkey" will be added to your path and a shortcut and desktop entry to the browser will be created.

To uninstall, either launch this MSI installer again and select uninstall, or go to add or remove programs in Windows settings.

You can now move onto the [usage](#usage) section.

### Linux

For Linux users, the first step is to clone this repository like so:

```sh
$ git clone https://campus.cs.le.ac.uk/gitlab/ug_project/22-23/srb55
```
Then navigate into the installer directory and run the install script.

```sh
$ cd srb55/installer
$ bash install.sh
```

Zonkey should now be added to your path, which can be verified using the command below. If you are using a desktop environment such as Gnome or KDE, you should see a desktop icon in your start menu.

```sh
$ zonkey --version
```

To uninstall at a later date, nagivate to this directory again and run the uninstall script.

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

The source code for the documentation and learning material is located under `interpreter/assets/documentation`.

## Repository Organisation

- `zonkey/` is the package for the browser and command line interface to Zonkey. It also contains `zonkey/wix`, which is a subfolder containing the configuration for creating the installer on Windows.
- `interpreter/` is the package for the interpreting zonkey code. It also contains the **documentation** source code and key **zonkey scripts** such as the calculator under `interpreter/assets/`, as these need to be accessible to native calls in Zonkey scripts.
- `installer/` contains the files required for installing the application on Windows and Linux.
- `fuzz-test/` contains the code for generating the fuzz testing binary.
- `editor-config/` contains the configuration for syntax highlighting in Vim/Neovim. This needs to be installed under the vim syntax folder, which is detailed [here](https://neovim.io/doc/user/syntax.html).
- `benchmarks/` contains the scripts and programs for benchmarking the Zonkey language, along with my results.

## Compilation 

A functioning installation of a Rust toolchain is required to compile Zonkey. Instructions on how to set this up can be found at <https://www.rust-lang.org/tools/install>.

Please be aware that the initial compilation of Zonkey can take a significant amount of time, around 15-30 minutes depending on your hardware.

There should be no need for additional requirements as the necessary development headers are included with Windows, Ubuntu, and Fedora. However, if your compilation is failing, please read the failed compilation output as it will detail what the development headers it could not find, as this may happen on minimal distributions of Linux.

Once you have a functioning toolchain for Rust, first clone the repository:

```sh
$ git clone https://campus.cs.le.ac.uk/gitlab/ug_project/22-23/srb55
```

Navigate into the directory and start the compilation.
```sh
$ cd srb55
$ cargo build --release --package zonkey
```
Once this has completed, the binary will be located at `target/release/zonkey`. To install this to your system path, you can do:

```sh
$ cargo install --path zonkey
$ zonkey --version
```

To run it the program without installing to your path, you can do the following.
```sh
$ cargo run -r -p zonkey -- --version
# Or
$ ./target/release/zonkey --version
```

If you are interested in testing the debug mode of Zonkey, which prints the current state of the interpreter during execution, you can try using the debug build profile. This will need will go through the compiliation process again, but shouldn't take as long as unoptimised code is being generated.

```sh
cargo build --profile debug-info -p zonkey
```

This will be located at `target/debug-info/zonkey` and can be run like so.

```sh
$ cargo run --profile debug-info -p zonkey
# Or
$ ./target/debug-info/zonkey
```

## Testing

To run the automatic test suites of Zonkey, you must first be able to compile the project as discussed in the [compliation section](#compilation).

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
Then build the fuzz-testing profile of my project.

```sh
cargo afl build -p fuzz-test --profile fuzz-testing
```
Finally, start the fuzz tester, feeding it some scripts to seed with in one of my testing directories.
```sh
cargo afl fuzz -i interpreter/src/tests/scripts -o out target/fuzz-testing/fuzz-test
```

### News Server

The news server requires Python and Flask to be installed to the system. Navigate to the `example-news-server` directory and run the server:
```
$ cd example-news-server
$ python server.py
```
Ensure that port 8000 is free. Then in the browser, you can either click the link on the homepage to `Binary Bulletin`, or type `http://localhost:8000` in the address bar to load the homepage. To clean the comments added, you can run `http://localhost:8000/clean`.

Optionally, if you want the server to go faster, you can use Waitress (requires installation through pip).
```sh
$ cd example-news-server
$ python waitress_server.py
```

## Benchmarking

Results from my testing are already available in `benchmarks/results`.

If you want to carry out the benchmarks yourself, it requires the following compilers and interpreters to be installed, along with some additional software. Note that benchmarking only works on Linux.

- Rustc (will be included with the Rust toolchain)
- Clang
- GCC
- Python
- Bun
- Deno
- Lox (has to be manually compiled from their repository and added to path)
- Zonkey itself installed to the path.
- Perf
- Diffuse for verification

Once these are installed, navigate to the benchmarking directory, and run a benchmark using the testing script. Specify the test, the number of cycles and optionally add Lox at the end to include it in the benchmarking test if a script has been created for it. Results will overwrite mine inside the `benchmarks/results` directory.
```sh
$ cd benchmarks
$ bash tester.sh hello_world 100 lox
$ bash tester.sh fibonacci 10 lox
$ bash tester.sh leibniz 10
$ bash tester.sh objects_arrays 10
```
You can also verify that each program is creating the same output like so.

```sh
$ bash verifier.sh hello_world lox
$ bash verifier.sh fibonacci lox
$ bash verifier.sh leibniz lox
$ bash verifier.sh objects_arrays lox
```

## Third Party Code Used

Third party crates are automatically downloaded by Cargo when you compile my project, and these dependencies to download are defined within the Cargo.toml file specified in each of my Rust packages.

I will link to the websites of these crates below, however, I would first like to mention that the svg's for the icons in the web browser, defined in `zonkey/src/tab/control_bar.rs`, were sourced from [Remix Icon](https://remixicon.com/), licensed under Apache License 2.0.

### Crates Used
- [clap](https://crates.io/crates/clap): MIT/Apache-2.0, used for the command line arguments parser.
- [non-empty-vec](https://crates.io/crates/non-empty-vec): MIT, a data structure used to ensure that the vector that contains the browser history can never be empty.
- [assert-cmd](https://crates.io/crates/assert-cmd): MIT/Apache-2.0, used for running integration tests by running the "zonkey" binary and asserting its stdout/stderr.
- [winapi](https://crates.io/crates/winapi): MIT/Apache-2.0, used to access the Windows API to close the console window on browser launch.
- [numtoa](https://crates.io/crates/numtoa): MIT/Apache-2.0, used to efficiently convert integers to bytes to print on the command line.
- [ryu](https://crates.io/crates/ryu): Apache-2.0/BSL-1.0, used to efficiently convert floats to bytes to print on the command line.
- [rustc-hash](https://crates.io/crates/rustc-hash): MIT/Apache-2.0, used as an efficient hashmap in the parser.
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation): MIT/Apache-2.0, used to break a string into graphemes for the lexer to operate on.
- [iced](https://crates.io/crates/iced): MIT, used as the GUI library to create the browser.
- [colorsys](https://crates.io/crates/colorsys): MIT, used as a helpful function to convert string hex codes into RGB8.
- [reqwest](https://crates.io/crates/reqwest): MIT/Apache-2.0, used for the sending and receiving information over http/https.
- [include-dir](https://crates.io/crates/include-dir): MIT, used to include assets in the binary of the executable, such as inbuilt zonkey scripts.
- [directories-next](https://crates.io/crates/directories-next): MIT/Apache-2.0, used to find the correct paths on Linux and Windows for storing installed Zonkey applications and locating desktop folders.
- [afl](https://crates.io/crates/afl): Apache-2.0, used as the fuzz tester for my lexer and parser.
- [cargo-wix](https://crates.io/crates/cargo-wix):  MIT/Apache-2.0, used to create the installer for Windows.
