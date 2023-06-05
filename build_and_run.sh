#!/bin/bash
#IMPORTANT
#change ttyACM0 for your port
cargo build
sudo chmod 666 /dev/ttyACM0 
#export RUST_BACKTRACE=1
espflash --monitor /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/xiaio-esp32c3-rust
