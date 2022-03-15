# top_evm_runtime

## Directories Instructions:
#### xevm_runtime
* language: `C++`
* target: `libxevm_runtime.a`
#### xevm_engine_rs:
* language: `Rust`
* target: `libxevm_engine.a`
#### tests
* language: `C++`
* target: `test_aurora`

## Build Essential:
* cmake: v3.8 or higher
* rust toolchain  
    * [start at here](https://www.rust-lang.org/)
* gtest lib
    * ubuntu: `sudo apt install libgtest-dev`
    * centos7: `...`

## HOW TO RUN TEST:
``` BASH
mkdir -p cbuild && cd cbuild
cmake .. && make -j4
# release:
# cmake .. -DCMAKE_BUILD_TYPE=Release && make -j4
./bin/Linux/test_aurora
```

## MIGHT BE USEFUL CONFIG:
``` BASH
$ cat ~/.gitconfig 
[url "git@github.com:"]
        insteadOf = https://github.com/

$ cat ~/.cargo/config 
[net]
git-fetch-with-cli = true
```