use esp_idf_hal::delay;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use uln2003::{StepperMotor, ULN2003};

struct Delay;
impl embedded_hal::blocking::delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        delay::FreeRtos::delay_ms(ms);
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let mut motor = ULN2003::new(
        PinDriver::output(peripherals.pins.gpio19).unwrap(),
        PinDriver::output(peripherals.pins.gpio18).unwrap(),
        PinDriver::output(peripherals.pins.gpio5).unwrap(),
        PinDriver::output(peripherals.pins.gpio17).unwrap(),
        Some(Delay),
    );

    // run for 100 steps with 5 ms between steps
    motor.step_for(100, 5).unwrap();

    // manually do steps
    loop {
        motor.step().unwrap();
        delay::FreeRtos::delay_ms(5);
    }
}
