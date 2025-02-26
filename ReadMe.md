# Intro

Setting up and working with esp32-s3-devkitc-1

- [Blink LED](src/blink.rs)

### Wokwi demo

https://wokwi.com/projects/420733216026365953  
![esp32-s3-led](https://github.com/user-attachments/assets/c65d45be-e51d-4fc4-a533-dc20ca27dac7)


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
[3] <https://www.rose-lighting.com/wp-content/uploads/sites/53/2020/05/SK68XX-MINI-HS-REV.04-EN23535RGB-thick.pdf>  
