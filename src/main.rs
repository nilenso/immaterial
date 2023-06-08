use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::{delay, gpio};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use futures::executor::block_on;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
    let peripherals = Peripherals::take().unwrap();
    let dabba: Rc<RefCell<gpio::Gpio15>> = Rc::new(RefCell::new(peripherals.pins.gpio15));
    let something = asdf(dabba);
    block_on(something);
}

fn blink(peripherals: Peripherals) {
    let mut led = gpio::PinDriver::output(peripherals.pins.gpio2).unwrap();

    loop {
        led.set_high().unwrap();
        delay::Ets::delay_ms(500);
        led.set_low().unwrap();
        delay::Ets::delay_ms(500);
    }
}

async fn asdf(dabba: Rc<RefCell<gpio::Gpio15>>) {
    let pin = dabba.borrow_mut();
    let sensor = gpio::PinDriver::input_output(pin).unwrap();

    let mut output_sensor = sensor.into_output().unwrap();
    dht_init_signal(&mut output_sensor);

    let mut input_sensor = output_sensor.into_input().unwrap();

    let v = sample_dht_signal(&mut input_sensor);
    for dur in v.iter() {
        print!("{:?}\t", dur);
    }
}

fn dht_init_signal(sensor: &mut gpio::PinDriver<gpio::Gpio15, gpio::Output>) {
    println!("Starting the init signal");
    // Set the pin to low for 10 ms
    sensor.set_low().unwrap();
    delay::Ets::delay_ms(10);

    // Set the pin to high for 40 microseconds
    sensor.set_high().unwrap();
    sensor.set_low().unwrap();
}

fn print_level(levels: Vec<gpio::Level>) {
    for level in levels.iter() {
        print!("{:?}", level);
    }
}

fn sample_dht_signal(sensor: &mut gpio::PinDriver<'_, gpio::Gpio15, gpio::Input>) -> Vec<gpio::Level> {
    let mut timer = 6400;
    let mut v : Vec<gpio::Level> = Vec::new();
    while timer > 0 {
        timer -= 1;
        let level = sensor.get_level();
        v.push(level);
        delay::Ets::delay_us(1);
    }
    return v
}

async fn dht_signal(sensor: &mut gpio::PinDriver<'_, gpio::Gpio15, gpio::Input>) -> u128 {
    sensor.wait_for_rising_edge().await.unwrap();
    let start = SystemTime::now();

    sensor.wait_for_falling_edge().await.unwrap();
    let duration = SystemTime::now().duration_since(start).unwrap().as_micros();
    duration 
}
