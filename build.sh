#!/bin/bash

chmod +x -- "$0"
sudo apt-get install build-essential cmake git libjson-c-dev libwebsockets-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
cd ttyd_tools/build
cmake ..
make && sudo make install
cd ..
cd ..