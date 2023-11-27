#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    blog_os::init();

    fn stack_overflow() {
        stack_overflow()
    }
    // uncomment to trigger stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {
        use blog_os::print;
        print!("_");

        for _ in 0..10000 {}
    }
}

// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
