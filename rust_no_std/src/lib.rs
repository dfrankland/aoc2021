#![no_std]

use libc_print::libc_eprintln;

#[panic_handler]
pub fn my_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
      libc_eprintln!("panicked at {}:{}:{}", location.file(), location.line(), location.column());
    }
    loop {}
}
