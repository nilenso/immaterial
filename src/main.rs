use anyhow::{bail, Result};
use core::str;
use embedded_svc::{
    http::{client::Client},
    io::Read,
};
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::client::{Configuration, EspHttpConnection},
};
use log::info;
mod wifi;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
//mod sense;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    info!("Hello, world!");

    // The constant `CONFIG` is auto-generated by `toml_config`.

    // Connect to the Wi-Fi network
    let wifi_conn = match wifi::wifi("Cornerstone", "Adiosbitch", peripherals.modem, sysloop) {
        Ok(inner) => inner,
        Err(err) => {
            // Red!
            bail!("cant connect to Wi-Fi network: {:?}", err)
        }
    };

    let mut led = gpio::PinDriver::output(peripherals.pins.gpio2).unwrap();
    led.set_high().unwrap();
    get("http://www.mobile-j.de//")?;

    loop {
        println!("Here: {:?}", wifi_conn.ap_netif().get_ip_info());
        //sense::sense();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn get(url: impl AsRef<str>) -> Result<()> {
    // 1. Create a new EspHttpClient. (Check documentation)
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    // 2. Open a GET request to `url`
    let request = client.get(url.as_ref())?;

    // 3. Submit write request and check the status code of the response.
    // Successful http status codes are in the 200..=299 range.
    let response = request.submit()?;
    let status = response.status();

    println!("Response code: {}\n", status);

    match status {
        200..=299 => {
            // 4. if the status is OK, read response data chunk by chunk into a buffer and print it until done
            let mut buf = [0_u8; 256];
            let mut reader = response;
            loop {
                if let Ok(size) = Read::read(&mut reader, &mut buf) {
                    if size == 0 {
                        break;
                    }
                    // 5. try converting the bytes into a Rust (UTF-8) string and print it
                    let response_text = str::from_utf8(&buf[..size])?;
                    println!("{}", response_text);
                }
            }
        }
        _ => bail!("Unexpected response code: {}", status),
    }

    Ok(())
}
