#!/bin/bash
set -e
docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -t bevy_win cargo build --target=x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/debug/bevy_android_mines.exe .
./bevy_android_mines.exe
