use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use futures::executor::block_on;
use log::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::time::{Duration, SystemTime};

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
        thread::sleep(Duration::from_millis(500));
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}

async fn asdf(dabba: Rc<RefCell<gpio::Gpio15>>) {
    let pin = dabba.borrow_mut();
    let mut sensor = gpio::PinDriver::input_output(pin).unwrap();
    dht_init_signal(&mut sensor);
    let mut i = 0;
    println!("Listening for data from DHT");

    while i < 42 {
        i += 1;
        let res = dht_signal(&mut sensor);
        if res.await {
            println!("true");
        } else {
            println!("false");
        }
    }
}

fn dht_init_signal(sensor: &mut gpio::PinDriver<gpio::Gpio15, gpio::InputOutput>) {
    println!("Starting the init signal");
    // Set the pin to low for 10 ms
    sensor.set_low().unwrap();
    thread::sleep(Duration::from_millis(10));

    // Set the pin to high for 40 microseconds
    sensor.set_high().unwrap();
    thread::sleep(Duration::from_micros(50));
    println!("Init signal finished");

    if sensor.is_low() {
        println!("Sensor pulled low");
    }

    sensor.set_low().unwrap();
    if sensor.is_low() {
        println!("We sent low");
    }
}

async fn dht_signal(sensor: &mut gpio::PinDriver<'_, gpio::Gpio15, gpio::InputOutput>) -> bool {
    sensor.wait_for_rising_edge().await.unwrap();
    let start = SystemTime::now();
    println!("Rising Edge detected");

    sensor.wait_for_falling_edge().await.unwrap();
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Falling Edge detected");

    println!("Duration: {} microseconds", duration.as_micros());
    if duration.as_micros() > 40 {
        return true;
    } else {
        return false;
    }
}
