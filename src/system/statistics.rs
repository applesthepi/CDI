use crate::vga_buffer::{cprint, cprintln, Color, ColorCode};
use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;

pub fn init() {
	unsafe {
		for i in 0..4 {
			STATISTICS.cpu_usage.lock().push(0.0);
		}
	}
}

pub fn update() {
	unsafe {
		*STATISTICS.memory_usage.lock() = crate::allocator::get_memory_usage();
	}
}

pub struct Statistics {
	memory_usage: Mutex<f64>,
	cpu_usage: Mutex<Vec<f64>>,
}

impl Statistics {
	pub const fn new() -> Self {
		Self {
			memory_usage: Mutex::new(0.0),
			cpu_usage: Mutex::new(Vec::new()),
		}
	}

	pub fn to_vga(&self) {
		cprint(
			ColorCode::new(Color::Green, Color::Black),
			format_args!(
				"====================================\nMemory {}%\n",
				*self.memory_usage.lock()
			),
		);

		for (i, x) in self.cpu_usage.lock().iter().enumerate() {
			cprint(
				ColorCode::new(Color::Green, Color::Black),
				format_args!("CPU{} {}%\n", i, *x),
			);
		}

		cprintln(
			ColorCode::new(Color::Green, Color::Black),
			format_args!("===================================="),
		);
	}

	pub fn to_serial(&self) {}
}

pub static mut STATISTICS: Statistics = Statistics::new();
