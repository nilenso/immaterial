use dht_sensor::{dht22, DhtReading};
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use std::thread;
use std::time::Duration;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pin: gpio::Gpio19 = peripherals.pins.gpio19;
    sense(pin);
}

fn sense(pin: gpio::Gpio19) {
    let mut sensor = gpio::PinDriver::input_output(pin).unwrap();

    sensor.set_high();
    thread::sleep(Duration::from_millis(1000));

    let mut d = delay::Ets;
    let reading = dht22::Reading {
        temperature: 0.0,
        relative_humidity: 0.0,
    };

    let mut input_sensor = sensor.into_input().unwrap();

    match dht22::Reading::read(&mut d, &mut sensor) {
        Ok(r) => println!("Got some reading"),
        Err(e) => println!("Failed with error"),
    }
}
