#!/bin/bash
if [ -f "./Makefile.toml" ]; then 
    OPENCLOUD_DIR = $PWD
else 
    git clone https://github.com/Rheydskey/OpenCloud.git opencloud 
    cd opencloud 
    OPENCLOUD_DIR = $PWD
fi
cd "${OPENCLOUD_DIR}" 
if [ command -v "cargo" ]; then 
    echo "Cargo install found !"
elif 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh 
    echo "Installing cargo..."
    bash rustup.sh -y 
    source $HOME/.cargo/env
fi
cargo install cargo-make
cargo make build_release