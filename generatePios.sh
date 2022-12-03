#!/bin/bash
ROOT_PATH="/home/fra_ub22/Projects/PiOS/Kernel"
KERNEL_PATH=$ROOT_PATH'/kernel'
BUILD_PATH=$ROOT_PATH'/build'
RELEASE_PATH=$BUILD_PATH'/target/thumbv7em-none-eabihf/debug'
DESTINATION_PATH='/home/fra_ub22/Desktop/Parallels Shared Folders/pios-shared-folder/rust-libraries'

cd $BUILD_PATH
cargo build
# cargo build --release
cd $RELEASE_PATH
cp -p libpios.a "$DESTINATION_PATH"'/binaries/'

cd $KERNEL_PATH
cbindgen --config cbindgen.toml --crate kernel --output pios.h
cp -p pios.h "$DESTINATION_PATH"'/includes/'
