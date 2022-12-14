#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hell::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hell::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("HESHER WAS HERE!");

    hell::init();

    #[cfg(test)]
    test_main();

    hell::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hell::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hell::test_panic_handler(info)
}

#[test_case]
fn simple_assertion() {
    assert_eq!(1, 1);
}
