#!/bin/bash

chmod +x -- "$0"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
source $HOME/.cargo/env
./ttyd_tools/build/ttyd -t cursorStyle=bar -t lineHeight=1.5 -t disableLeaveAlert=true -t disableResizeOverlay=true-t disableReconnect=true-t titleFixed=rust-wordle -t fontSize=28 bash start.sh