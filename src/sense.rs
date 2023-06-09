use dht_sensor::dht22;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::Peripherals;


pub fn sense() {
    let peripherals = Peripherals::take().unwrap();
    let pin: gpio::Gpio15 = peripherals.pins.gpio15;
    let mut sensor: gpio::PinDriver<gpio::Gpio15, gpio::InputOutput> =
        gpio::PinDriver::input_output_od(pin).unwrap();

    sensor.set_high().unwrap();

    let mut d: delay::Ets = delay::Ets;

    match dht22::read(&mut d, &mut sensor) {
        Ok(r) => println!(
            "Temperature: {}\tHumidity: {}",
            r.temperature, r.relative_humidity
        ),
        Err(e) => println!("Failed with error: {:?}", e),
    }
}
