# machine_stack

Prototype implementation of the Machine Stack

# Installation

    sudo apt install clang
    wget https://github.com/cartesi/image-kernel/releases/download/v0.20.0/linux-6.5.13-ctsi-1-v0.20.0.bin \
    -O ./linux.bin
    wget https://github.com/cartesi/machine-emulator-tools/releases/download/v0.16.1/rootfs-tools-v0.16.1.ext2 \
    -O ./rootfs.ext2

# Install cross compilation

- install docker and add user to docker group (restart after). Then

    cargo install cross --git https://github.com/cross-rs/cross

- inside one of the app folders (like test-cross), run

    cross build --target riscv64gc-unknown-linux-gnu
    mkdir exec
    cp ./test-cross/target/riscv64gc-unknown-linux-gnu/debug/test-cross exec

# assembling the file system

    sudo apt install e2tools
    e2cp -P 755 ./test-cross/target/riscv64gc-unknown-linux-gnu/debug/test-cross rootfs.ext2:/home/dapp
