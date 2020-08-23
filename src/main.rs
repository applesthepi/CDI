#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CDI::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::Page;
use x86_64::{structures::paging::MapperAllSizes, VirtAddr};
use CDI::allocator;
use CDI::memory;
use CDI::memory::translate_addr;
use CDI::memory::BootInfoFrameAllocator;
use CDI::println;
use CDI::system;
use CDI::vga_buffer::ColorCode;
use CDI::vga_buffer::{cprintln, Color};

extern crate alloc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	CDI::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	////////////////////////////////////////
	// PRE Initialization
	////////////////////////////////////////

	CDI::init();

	////////////////////////////////////////
	// Initialization
	////////////////////////////////////////

	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	////////////////////////////////////////
	// POST Initialization
	////////////////////////////////////////

	system::init();

	////////////////////////////////////////
	// Program
	////////////////////////////////////////

	let heap_memory = Vec::<u8>::with_capacity(55555);

	system::update(); // will happen on another thread in the future
	unsafe { system::statistics::STATISTICS.to_vga() };

	CDI::hlt_loop();
}

entry_point!(kernel_main);
