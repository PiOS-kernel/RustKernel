# PiOS Kernel

## Project structure

- The kernel source code lives in the [kernel](kernel) folder.
- The [test_app](test_app) contains an application to run tests on qemu.
- The [build](build) folder is used to build PiOS as a static library (more on that in the next section).

## Building the kernel

Pios is provided as a static library, issue the following commands to build it (assuming you are currently in the root directory of the project):
```
$ cd build
$ cargo build --release
$ cp target/thumbv7em-none-eabi/release/libpios.a ..
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