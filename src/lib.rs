#![no_std]
#![allow(clippy::empty_loop)]
#![deny(warnings)]

use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m::interrupt;

struct RAM
{
	offset: u32,
}
impl core::fmt::Write for RAM
{
	fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error>
	{
		extern "C"
		{
			static mut __sbss: u8;
		}
		let dat = s.as_bytes();
		let len = dat.len();
		unsafe
		{
			core::ptr::copy(
				dat.as_ptr() as *mut u8,
				(&mut __sbss as *mut u8).offset(self.offset as isize),
				len
			)
		};
		self.offset += len as u32;
		Ok(())
	}
}
//This should be removed if the kernel has a panic-handler built-in.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
	interrupt::disable();
	writeln!(RAM { offset: 0 }, "{}", info).ok();
	loop {}
}
