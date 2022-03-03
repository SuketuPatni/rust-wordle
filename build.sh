#!/bin/bash

apt-get install build-essential cmake git libjson-c-dev libwebsockets-dev
cd ttyd_tools/build
cmake ..
make && sudo make install
cd ..
cd ..