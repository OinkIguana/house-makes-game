# House Makes Game (in Rust)

To develop/run, first install Rust by visiting [https://rustup.rs/](https://rustup.rs/) and following the instructions. 
This will install 3 programs:
1.  `rustup`: a program used to install and update Rust again later (by running `rustup update`)
2.  `rustc`: the Rust compiler (you'll never need to touch this yourself)
3.  `cargo`: the tool used to actually compile/run/work with Rust code

After that is installed, you can use the following commands (in this directory) to run the game:

```sh
# compile everything, but don't run
cargo build

# compile everything and run it
cargo run

# compile all relevant documentation (our documentation, as well as for all the packages we use)
cargo doc

# run all unit tests (i.e. do nothing because we have no unit tests)
cargo test
```

External packages we use are managed by Cargo also. They will be downloaded and installed automatically when you build.
To see these packages and other configuration for this project, see `Cargo.toml`. If you need to add another package 
for something, just add it to the list in `Cargo.toml` and next time you build it will be installed.

If you decide to run `cargo doc`, the documentation for each package will be generated in 
`target/doc/{package_name}/`, and can be viewed in your browser.

The code is all in the `src/` directory. `main.rs` is where the `main` function is.

To learn all of Rust, read the [Rust book](https://doc.rust-lang.org/book/). To learn about the libraries we are using, 
see the documentation (as compiled above) or find it online by searching [docs.rs](https://docs.rs/). You can find 
documentation for the Rust standard library [here](https://doc.rust-lang.org/std/).

## Dependencies

### Windows

You will need to have installed MSVC before you will be able to compile anything. MSVC comes with Visual Studio, so make
sure that is installed (2019 Community Edition worked fine for me. Pretty sure any other version will too)

### Mac

Seems ok so far for me.

### Linux (Ubuntu)

In order to compile, you might need to install some dependencies:

```
sudo apt install libasound2-dev libudev-dev
```
