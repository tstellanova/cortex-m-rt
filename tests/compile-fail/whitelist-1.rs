#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception, interrupt};

#[inline] //~ ERROR this attribute is not allowed on a cortex-m-rt entry point
#[entry]
fn foo() -> ! {
    loop {}
}

#[inline] //~ ERROR this attribute is not allowed on an exception handler controlled by cortex-m-rt
#[exception]
fn SysTick() {}

#[allow(non_camel_case_types)]
enum interrupt {
    USART1,
    USART2,
}

#[inline] //~ ERROR this attribute is not allowed on an interrupt handler controlled by cortex-m-rt
#[interrupt]
fn USART1() {}

#[cfg(feature = "device")]
#[cfg_attr(feature = "device", inline)] //~ ERROR this attribute is not allowed on an interrupt handler controlled by cortex-m-rt
#[interrupt]
fn USART2() {}
