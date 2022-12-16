# PiOS Kernel

## Project structure

- The kernel source code lives in the [kernel](kernel) folder.
- The [test_app](test_app) contains an application to run tests on qemu.
- The [build](build) folder is used to build PiOS as a static library (more on that in the next section).

## Building the kernel

Building the kernel requires a nightly version of the compiler. To install that run:
```
$ rustup override set nightly
```
Now the default compiler that cargo will use to build this project is set to the nightly version.

Pios is provided as a static library, issue the following commands to build it (assuming you are currently in the root directory of the project):
```
$ cd build
$ cargo build --release
$ cp target/thumbv7em-none-eabihf/release/libpios.a ..
$ cd ..
```

To generate `.h` file using `cbindgen`
```
$ cd kernel
$ cbindgen --config cbindgen.toml --crate kernel --output pios.h
$ cp kernel/pios.h ..
$ cd ..
```

In the root directory you should now have a `libpios.a` file.

## Testing

All unit and integration tests are run on a qemu virtual machine. To run tests, type the following commands in your terminal:
```
$ cd test_app
$ cargo test
```
that should open a qemu terminal, where the results of each test that was run is displayed. To exit type `ctrl + A`, and then `X`.

### Writing a test

The code for tests is found inside the [test_app/src](test_app/src) directory. In the [main.rs](test_app/src/main.rs) the functions needed to run the tests and the binary entrypoint are defined, those should not change.

Tests are separated into different files, according to the kernel module they are associated to. For example, to write a new test for the `allocator` module, just edit the [allocator_tests.rs](test_app/src/allocator_tests.rs) file by adding the following code:

```
#[test_case]
fn your_test() {
    ...code...
}
```

The `#[test_case]` directive lets the compiler know that that function is a test that should be run when you issue the `cargo test` command.

## Debugging with GDB

In some situations the debugger is extremely helpful to debug kernel code. To run tests with debugger support, edit the [config.toml](./test_app/.cargo/config.toml) file by uncommenting the appropriate runner:
```
# to run without gdb support
# runner = ...

# to run with gdb support
runner = ...
```

Now you need to set the path to the executable that GDB should look for. In the [.gdbinit](./test_app/.gdbinit) file set the `file` path to the test executable. Be careful, the test executable is not [test_app](./test_app/target/thumbv7em-none-eabi/debug/test_app), it is instead found in the [deps](./test_app/target/thumbv7em-none-eabi/debug/deps/) directory, by the name `test_app-` followed by a string of digits.

To run the tests, as usual:
```
$ cd test_app
$ cargo test
```
the execution is halted immediately, waiting for probes from the debugger. Now run the debugger by opening another terminal and typing:
```
$ cd test_app
$ gdb-multiarch
```
the first time you run the debugger a warning will be displayed asking you to add the path to the [.gdbinit](./test_app/.gdbinit) file to a gdb-specific configuration file. You should do that. If the directory containing the configuration file does not exist, just create it and add the configuration file, than inside it write the command displayed by the warning message.You can now kill gdb and re-start it, the warning message should not be displayed anymore.

To get some inspiration on the capabilities of gdb, have a look at the commands listed in the [.gdbinit](./test_app/.gdbinit) file.