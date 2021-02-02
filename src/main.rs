#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CDI::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fat32::base::Volume;
use sdio_sdhc::sdcard::Card;
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
	stm
	fn gpio_init(rcc: &mut stm32::RCC, gpioc: &mut stm32::GPIOC, gpiod: &mut stm32::GPIOD) {
		// gpioc gpiod enable
		rcc.ahb1enr
			.modify(|_r, w| w.gpiocen().set_bit().gpioden().set_bit());

		gpioc.afrh.modify(|_r, w| {
			w.afrh8()
				.af12()
				.afrh9()
				.af12()
				.afrh10()
				.af12()
				.afrh11()
				.af12()
				.afrh12()
				.af12()
		});
		gpiod.afrl.modify(|_r, w| w.afrl2().af12());

		gpioc.moder.modify(|_r, w| {
			w.moder8()
				.alternate()
				.moder9()
				.alternate()
				.moder10()
				.alternate()
				.moder11()
				.alternate()
				.moder12()
				.alternate()
		});
		gpiod.moder.modify(|_r, w| w.moder2().alternate());

		gpioc.ospeedr.modify(|_r, w| {
			w.ospeedr8()
				.high_speed()
				.ospeedr9()
				.high_speed()
				.ospeedr10()
				.high_speed()
				.ospeedr11()
				.high_speed()
				.ospeedr12()
				.high_speed()
		});
		gpiod.ospeedr.modify(|_r, w| w.ospeedr2().high_speed());

		gpioc.otyper.modify(|_r, w| {
			w.ot8()
				.push_pull()
				.ot9()
				.push_pull()
				.ot10()
				.push_pull()
				.ot11()
				.push_pull()
				.ot12()
				.push_pull()
		});
		gpiod.otyper.modify(|_r, w| w.ot2().push_pull());

		gpioc.pupdr.modify(|_r, w| {
			w.pupdr8()
				.pull_up()
				.pupdr9()
				.pull_up()
				.pupdr10()
				.pull_up()
				.pupdr11()
				.pull_up()
				.pupdr12()
				.pull_up()
		});
		gpiod.pupdr.modify(|_r, w| w.pupdr2().pull_up());
	}
	
	let card = Card::init().unwrap();
	println!("{:#?}", card);
	card.erase(0, card.capacity).unwrap();

	let buf = [1; 512 * 2];
	card.write_multi_blocks(&buf, 0, 2).unwrap();

	let mut buf = [0; 512 * 2];
	card.read_multi_blocks(&mut buf, 0, 2).unwrap();
	println!("{:?}", &buf[0..buf.len()]);

	let buf = [2; 512];
	card.write_block(&buf, 512).unwrap();

	let mut buf = [0; 512];
	card.read_block(&mut buf, 512).unwrap();
	println!("{:?}", &buf[0..buf.len()]);

	// Card from sdio_sdhc
	let card = Card::init().unwrap();
	// Volume from fat32
	let cont = Volume::new(card);
	// into root dir
	let root = cont.root_dir();
	// create file named test.txt
	root.create_file("test.txt").unwrap();
	// load file
	let mut file = root.load_file("test.txt").unwrap();
	// write buffer to file
	file.write(&[80; 512 * 9]).unwrap();

	let heap_memory = Vec::<u8>::with_capacity(55555);

	system::update(); // will happen on another thread in the future
	unsafe { system::statistics::STATISTICS.to_vga() };

	CDI::hlt_loop();
}

entry_point!(kernel_main);
