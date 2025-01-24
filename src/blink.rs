#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::{
    main,
    rmt::{Error, PulseCode, Rmt, TxChannel, TxChannelConfig, TxChannelCreator},
    time::RateExtU32,
};
use log::info;

const T0H_NS: u32 = 400;
const T0L_NS: u32 = 850;
const T1H_NS: u32 = 800;
const T1L_NS: u32 = 450;
const CPU_FREQ_MHZ: u32 = 80;
const T1H_CYCLE: u16 = (T1H_NS * CPU_FREQ_MHZ / 1000) as u16;
const T1L_CYCLE: u16 = (T1L_NS * CPU_FREQ_MHZ / 1000) as u16;
const T0H_CYCLE: u16 = (T0H_NS * CPU_FREQ_MHZ / 1000) as u16;
const T0L_CYCLE: u16 = (T0L_NS * CPU_FREQ_MHZ / 1000) as u16;

#[derive(Debug, Clone, Copy)]
enum Color {
    Green,
    Red,
    Blue,
}

// fill the data with patterns for the corresponding color
fn create_pattern(color: Color, data: &mut [u32; 25]) {
    let pattern = match color {
        Color::Red => 0b000000001111111100000000,
        Color::Green => 0b111111110000000000000000,
        Color::Blue => 0b000000000000000011111111,
    };
    for (i, entry) in data.iter_mut().enumerate().take(24) {
        let bit = (pattern >> (23 - i)) & 1u32;
        let value: u32 = if bit == 1u32 {
            PulseCode::new(true, T1H_CYCLE, false, T1L_CYCLE)
        } else {
            PulseCode::new(true, T0H_CYCLE, false, T0L_CYCLE)
        };
        *entry = value;
    }
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_println::logger::init_logger_from_env();
    let delay = Delay::new();

    // create an instance of RMT peripheral
    let rmt = Rmt::new(peripherals.RMT, CPU_FREQ_MHZ.MHz()).unwrap();

    // Configure channel
    let mut channel = rmt
        .channel0
        .configure(
            peripherals.GPIO48,
            TxChannelConfig {
                clk_divider: 1,
                idle_output_level: false,
                idle_output: false,
                ..Default::default()
            },
        )
        .unwrap();

    let mut data = [PulseCode::empty(); 25];
    // end marker pattern is mandatory
    data[data.len() - 1] = PulseCode::empty();

    info!("Starting loop..");

    let colors = [Color::Red, Color::Green, Color::Blue];
    let mut index = 0;
    loop {
        let color = colors[index];
        info!("Color is {:?}", color);
        create_pattern(color, &mut data);
        // transmit consumes the channel
        match channel.transmit(&data) {
            Ok(transaction) => {
                match transaction.wait() {
                    Ok(reclaimed_channel) => {
                        channel = reclaimed_channel;
                        delay.delay_millis(2000);
                        index = (index + 1) % colors.len();
                    }
                    Err((err, returned_channel)) => {
                        // Handle specific transmission errors
                        match err {
                            Error::TransmissionError => info!("Transmission failed"),
                            Error::EndMarkerMissing => info!("End marker missing"),
                            Error::Overflow => info!("overflow"),
                            Error::UnreachableTargetFrequency => {
                                info!("UnreachableTargetFrequency")
                            }
                            Error::InvalidArgument => info!("InvalidArgument"),
                        }
                        channel = returned_channel;
                    }
                }
            }
            Err(e) => {
                panic!("Failed to start transmission: {:?}", e);
            }
        }
    }
}
