#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CDI::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CDI::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	CDI::init();

	//fn stack_overflow() {
	//	stack_overflow();
	//}
	//
	//stack_overflow();

	println!("It did not crash!");
	CDI::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	CDI::hlt_loop();
}
