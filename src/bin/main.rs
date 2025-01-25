#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Io, Level, Output},
    main,
};
use esp_println::println;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO8, Level::Low);

    let delay = Delay::new();

    println!("HELLO WORLD!");

    loop {
        println!("HELLO LOOP!");
        led.toggle();
        delay.delay_millis(1000);
    }
}