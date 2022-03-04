#!/bin/bash

chmod +x -- "$0"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
git clone https://github.com/tsl0922/ttyd.git ttyd_tools
cd ttyd_tools
rm -r .git
rm -r .github
rm .gitattributes
rm .gitignore
rm LICENSE
mkdir build && cd  build
cmake ..
make && make install
cd ..
cd ..
source $HOME/.cargo/env
ttyd -t cursorStyle=bar -t lineHeight=1.5 -t disableLeaveAlert=true -t disableResizeOverlay=true-t disableReconnect=true-t titleFixed=rust-wordle -t fontSize=28 bash start.sh