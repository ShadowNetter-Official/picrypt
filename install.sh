#!/bin/bash

echo
echo "picrypt"
echo "by ShadowNetter"
echo
echo "cloning into repo..."
git clone https://github.com/ShadowNetter-Official/picrypt
cd picrypt
echo "done"
echo "installing..."
cargo build --release
cp target/release/picrypt ~/.cargo/bin/
echo "done"
echo
echo "to uninstall do: "
echo "rm ~/.cargo/bin/picrypt"
echo
