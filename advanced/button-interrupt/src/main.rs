use anyhow::Result;
use dht11::Dht11;
use esp_idf_svc::{
    hal::{
        delay::Delay,
        gpio::{Gpio2, InterruptType, PinDriver, Pull},
        peripheral::Peripheral,
        peripherals::Peripherals,
        task::notification::Notification,
    },
    sys::{esp_random, GPIO},
};
use rgb_led::{RGB8, WS2812RMT};
use std::num::NonZeroU32;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let pin_d = PinDriver::input_output_od(peripherals.pins.gpio4)?;
    let mut dht11 = Dht11::new(pin_d);
    let mut delay = Delay::new(10);
    match dht11.perform_measurement(&mut delay) {
        Ok(meas) => println!("Temp: {} | Hum: {}", meas.temperature, meas.humidity),
        Err(e) => println!("Error: {:?}", e),
    }

    // 1. Configure the button using PinDriver
    // let mut button = PinDriver...

    // 2. Instantiate a new notification and notifier

    unsafe {
        // 3. Create a subscription and its callback function that notifies and yields.
    }

    loop {
        unsafe {
            // 4. Enable the interrupt for the button
            // 5. Wait for notification using `esp_idf_svc::hal::delay::BLOCK`
            // 6. Print a "button pressed" message
        }
    }
}
