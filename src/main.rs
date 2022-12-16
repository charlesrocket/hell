#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hell::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use hell::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hell::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("HESHER WAS HERE!");
    hell::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));

    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

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
