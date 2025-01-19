# Intro

Setting up and working with esp32-s3-devkitc-1

- [Blink LED](src/blink.rs)

## Pre-requisties

```shell
# Install espup for fetching the esp specific toolchain 
# (tier 3 targets are not available via rustup)
cargo install espup
espup install --targets=esp32s3
source ~/export-esp.sh
cargo run
```

## References

[1] <https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32s3/esp32-s3-devkitc-1/user_guide_v1.0.html>  
[2] <https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html>
