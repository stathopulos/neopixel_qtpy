<p align="center">
    Example crate for the CH32V203 version of Adafruit's QT Py.
</p>

<p align="center">
  <a href="#instructions">Instructions</a> |
  <a href="#board-info">Board Info</a> |
  <a href="#dependencies">
    Dependencies</a>
  &nbsp;&nbsp;
</p>

Support for the CH32 series of chips has become surprisingly good recently thanks to lots of great work to bring support for these and many other microcontrollers. These 32-bit chips are super cheap, small, fairly full-featured, and *relatively* fast, with the caveat that support and documentation for them tends to be more limited than other microcontrollers.

There are a few good reasons to use Rust over something like the Arduino framework for 32-bit platforms. Efficiency, better compile-time guarantees, a more modern language, more modern and flexible tooling, access to the Rust no_std ecosystem, a solid HAL, and async with Embassy all contribute to solid low level access and better high level control. A simple but more explicit build system, code that's no more complex, and barely more boilerplate (mostly more explicit dependencies than arduino standard libraries or importing whole .h files) mean not much cost by the programer for these benefits. Only 10kb are flashed for this program, 3x less than the 30kb for the comparable Arduino program.

> [!NOTE]
> This crate currently relies on a nightly toolchain and some early development crates

## Instructions
- Make sure you have Rust installed
- Clone this repo using `git clone https://github.com/fourMew/neopixel_qtpy.git`
- Install [wchisp](https://github.com/ch32-rs/wchisp) and its dependencies according to the instructions in their repo to use for flashing the program
- While holding down BOOT (the inner button), reset your board, either by connecting to USB or pressing the RST (outer) button
- Verify your chip is connected with `wchisp show` and use `cargo run --release` to compile and flash the program. The `rust-toolchain.toml` file should take care of getting the appropriate toolchain for you.

> [!IMPORTANT]
> Always compile using the release profile to ensure your program is small enough to fit in flash 

## Dependencies 
 - `ch32_hal`: Hardware Abstraction Layer providing convenient, unified, and safe bindings to low level hardware
 - `smart_leds`: Thin layer for convenient LED control and the `ws2812-delay` driver from [rappet](https://github.com/rappet/) based on delay from the hal 
 - a few odds and ends from core and the necessary hardware libraries

 ## Board Info
- [Qt Py docs](https://learn.adafruit.com/adafruit-qt-py-ch32v203/)
- [CH32 docs](https://www.wch-ic.com/products/CH32V203.html)
- [LED datasheet](https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf)
- [smart_leds documentation](https://docs.rs/smart-leds/latest/smart_leds/)