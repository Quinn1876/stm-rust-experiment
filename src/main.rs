#![no_std]
#![no_main]

use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low` function
use stm32f0xx_hal::{delay::Delay, pac, prelude::*}; // stm specific functions

#[allow(unused_imports)]
use panic_halt; // When a panic occurrs stop the microcontroller


// Marks the entry point to the applicaiton
// Cortex runtime adds some startup code before this to 
// help get us going
#[entry]
fn main() {
    
}
