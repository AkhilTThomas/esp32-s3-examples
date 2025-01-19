#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::main;
use log::info;

use bitfield::bitfield;

bitfield! {
    pub struct RGB(u32);
    impl Debug;
    u8;
    get_blue, set_blue: 7,0;
    get_red, set_red: 15,8;
    get_green, set_green: 23,16;
    unused, set_unused: 24,31;
}

pub struct SK6812<'a> {
    pin: Output<'a>,
    delay: Delay,
}

impl<'a> SK6812<'a> {
    pub fn new(pin: Output<'a>, delay: Delay) -> Self {
        Self { pin, delay }
    }
    #[inline(always)]
    fn send_bit(&mut self, bit: bool) {
        if bit {
            // T1H
            self.pin.set_high();
            self.delay.delay_nanos(1000);
            self.pin.set_low();
            self.delay.delay_nanos(500);
        } else {
            self.pin.set_high();
            self.delay.delay_nanos(400);
            self.pin.set_low();
            self.delay.delay_nanos(1100);
        }
    }

    fn send_byte(&mut self, byte: u8) {
        for i in (0..8).rev() {
            self.send_bit(byte & (1 << i) != 0);
        }
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.send_byte(green);
        self.send_byte(red);
        self.send_byte(blue);
        // Send Trst
        self.pin.set_low();
        self.delay.delay_micros(100);
    }
}

// Toggle the RGB lights controlled via GPIO48
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    //let mut rgb = RGB(0);
    let led_pin = Output::new(peripherals.GPIO48, Level::Low);
    let mut led = SK6812::new(led_pin, delay);
    loop {
        //set Red
        led.set_color(255, 0, 0);
        delay.delay_millis(1000);

        //set Green
        //led.set_color(0, 255, 0);
        //delay.delay_millis(1000);

        //set Blue
        //led.set_color(0, 0, 255);
        //delay.delay_millis(1000);
    }
}
