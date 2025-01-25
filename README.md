# esp32-c6-blink-led-example
Docs:
- https://docs.esp-rs.org/esp-hal/esp-hal/0.23.1/esp32h2/esp_hal/#blinky
- https://docs.esp-rs.org/book/

# Getting Started
- `rustup toolchain install nightly --component rust-src`
- `rustup target add riscv32imac-unknown-none-elf`
- `cargo install esp-generate`
- `esp-generate --chip=esp32c6 esp32-c6-blink-led-example`
- `cargo install espflash`

Once you have your code written, run `cargo build` to build the binary.

To flash the binary run: `espflash flash target/riscv32imac-unknown-none-elf/debug/esp32-c6-blink-led-example` or  `espflash flash target/riscv32imac-unknown-none-elf/debug/esp32-c6-blink-led-example --monitor` for direct feedback.

Personally, I ran into permissions errors from the USB port when trying to flash. To fix this I ran: `sudo chmod 666 /dev/ttyACM0`
