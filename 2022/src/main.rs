#![no_std]
#![no_main]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

mod util;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::StatefulOutputPin;
use fugit::ExtU32;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

static mut DELAY: Option<cortex_m::delay::Delay> = None;
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;
static mut TIMER: Option<hal::Timer> = None;

fn _write_usb_serial(value: &str, with_newline: bool) -> Option<()> {
    let mut formatted = heapless::String::<512>::new();

    // the serial terminal apparently wants '\r\n' as the line break.
    let mut prev: char = '\0';
    for ch in value.chars() {
        if ch == '\n' && prev != '\r' {
            formatted.push_str("\r\n");
        } else {
            formatted.push(ch);
        }
        prev = ch;
    }

    if with_newline {
        formatted.push_str("\r\n");
    }

    let serial = unsafe { USB_SERIAL.as_mut() }?;
    let bytes = formatted.as_bytes();
    let mut total_bytes_written = 0;

    // make sure all bytes are written.
    while total_bytes_written < bytes.len() {
        let chunk = &bytes[total_bytes_written..];
        if let Ok(res) = serial.write(chunk) {
            total_bytes_written += res;
        } else {
            // the USB buffer is full, or something.
            break;
        }
    }

    serial.flush();

    return Some(());
}

#[macro_export]
macro_rules! print {
    ($($x:tt)*) => {{
        use core::fmt::Write;
        use crate::_write_usb_serial;
        use heapless::String;

        let mut s = String::<512>::new();
        let _ = write!(&mut s, $($x)*);
        _write_usb_serial(&s, false);
    }};
}

#[macro_export]
macro_rules! println {
    ($($x:tt)*) => {{
        use core::fmt::Write;
        use crate::_write_usb_serial;
        use heapless::String;

        let mut s = String::<512>::new();
        let _ = write!(&mut s, $($x)*);
        _write_usb_serial(&s, true);
    }};
}

#[inline(never)]
#[panic_handler]
fn _panic_handler(info: &PanicInfo) -> ! {
    let timer = unsafe { TIMER.as_ref().unwrap() };
    let mut print_counter = timer.count_down();
    print_counter.start(5_000.millis());

    let mut k: u64 = 0;

    loop {
        poll_usb_serial();

        if let Ok(_) = print_counter.wait() {
            println!("########## PANIC {} ##########", k);
            println!("{}", info);
            k += 1;
        }

        atomic::compiler_fence(Ordering::SeqCst);
    }
}

fn run_day<T: core::fmt::Display>(day: util::Day<T>) {
    println!("########## AOC {} day {} ##########", day.year, day.day);

    for (i, test) in day.tests.iter().enumerate() {
        print!("Running test {}... ", i + 1);
        test();
        println!("OK");
    }

    for (i, part) in day.parts.iter().enumerate() {
        let result = part();
        println!("Part {}: {}", i + 1, result);
    }
}

fn poll_usb_serial() -> () {
    let usb_dev_ref = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial_ref = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev_ref.poll(&mut [serial_ref]) {
        return;
    }

    let mut buffer = [0u8; 256];
    serial_ref.read(&mut buffer);
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        USB_BUS = Some(usb_bus);
    }
    let usb_bus_ref = unsafe { USB_BUS.as_ref().unwrap() };
    let serial = SerialPort::new(usb_bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    let serial_ref = unsafe { USB_SERIAL.as_mut().unwrap() };
    let usb_dev = UsbDeviceBuilder::new(usb_bus_ref, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("42069 Blaze AB")
        .product("Serial port")
        .serial_number("PICO")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .max_packet_size_0(64)
        .build();
    unsafe {
        USB_DEVICE = Some(usb_dev);
    }

    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    unsafe {
        DELAY = Some(delay);
    }

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    unsafe {
        TIMER = Some(timer);
    }

    let timer_ref = unsafe { TIMER.as_ref().unwrap() };
    let mut led_pin = pins.led.into_push_pull_output();
    let led_freq = 100.millis();
    let mut led_counter = timer_ref.count_down();
    led_counter.start(led_freq);

    let mut try_usb_timer = timer_ref.count_down();
    try_usb_timer.start(1000.millis());

    let mut do_run_day_timer = timer_ref.count_down();
    do_run_day_timer.start(5_000.millis());

    let mut did_init_usb = false;

    loop {
        if let Ok(_) = led_counter.wait() {
            if led_pin.is_set_high().unwrap_or(false) {
                led_pin.set_low();
            } else {
                led_pin.set_high();
            }
        }

        poll_usb_serial();

        if !did_init_usb {
            if let Err(_) = try_usb_timer.wait() {
                continue;
            }
            if let Err(_) = serial_ref.write("Serial ready!\r\n".as_bytes()) {
                continue;
            }
            did_init_usb = true;
        }

        if let Ok(_) = do_run_day_timer.wait() {
            // run_day(day01::DAY_01);
            // run_day(day02::DAY_02);
            // run_day(day03::DAY_03);
            // run_day(day04::DAY_04);
            // run_day(day05::DAY_05);
            // run_day(day06::DAY_06);
            run_day(day07::DAY_07);
        }
    }
}
