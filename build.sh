#!/bin/bash

chmod +x -- "$0"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
source $HOME/.cargo/env
cd ttyd_tools/build
cmake ..
make && make install
cd ..
cd ..