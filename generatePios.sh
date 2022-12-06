#!/bin/bash
ROOT_PATH="/home/fra_ub22/Projects/PiOS/Kernel"
KERNEL_PATH=$ROOT_PATH'/kernel'
BUILD_PATH=$ROOT_PATH'/build'
DEBUG_PATH=$BUILD_PATH'/target/thumbv7em-none-eabihf/debug'
RELEASE_PATH=$BUILD_PATH'/target/thumbv7em-none-eabihf/release'
DESTINATION_PATH='/home/fra_ub22/Desktop/Parallels Shared Folders/pios-shared-folder/rust-libraries'

cd $BUILD_PATH
#cargo build
#cd $DEBUG_PATH
cargo build --release
cd $RELEASE_PATH
cp -p libpios.a "$DESTINATION_PATH"'/binaries/'

cd $KERNEL_PATH
cbindgen --config cbindgen.toml --crate kernel --output pios.h
cp -p pios.h "$DESTINATION_PATH"'/includes/'
