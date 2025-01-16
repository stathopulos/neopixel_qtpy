#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal as hal;
use core::iter::once;
use hal::delay::Delay;
use hal::gpio::{Level, Output, Speed};
use hal::println;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_delay::Ws2812;

#[qingke_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_144MHZ_HSI;
    let p = hal::init(config);

    let mut delay = Delay;

    let pin = Output::new(p.PA4, Level::Low, Speed::High);
    let mut led = Ws2812::new(Delay, pin);

    let color = RGB8 {
        r: 255,
        g: 0,
        b: 255,
    };

    let _ = led.write(once(color));

    delay.delay_ms(500);
    
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = println!("\n\n\n{}", info);

    loop {}
}
