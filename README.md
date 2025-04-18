# RustyBins
A CLI GTFOBins browser, for those who don't like leaving the terminal

## Important disclaimer

This project is a Rust-based rewrite of the original [ggtfobins](https://github.com/CristinaSolana/ggtfobins) tool by [Cristina Solana](https://github.com/CristinaSolana). All credit for the concept, design, and original implementation goes to her and the maintainers of the original repository. This version aims to replicate the functionality in Rust, with no intention to claim ownership of the original work.

## Installing

You will need to compile the code, don't worry as this is a very simple process:

First make sure you have installed cargo with you package manager of choice, then:

``` bash
git clone https://github.com/XoanOuteiro/rustybins
cd ./rustybins/
cargo build
```

Once the process is finished your binary named rustybins will be at ./target/debug/ , just add it to a folder that's in your PATH such as /usr/bin/

## Usage

You may consult one or many binaries for each exploit, for example:

``` bash
rustybins --exploit suid --bins vi,vim,nc
rustybins --exploit file-read --bins xargs
```
