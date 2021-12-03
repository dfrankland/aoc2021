#![no_std]
#![no_main]

use libc_print::libc_println;
#[allow(unused_imports)]
use rust_no_std::my_panic;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    main_result().unwrap();
    0
}

fn main_result() -> Result<(), ()> {
    const INPUT: &str = include_str!("../../inputs/day_1.csv");

    let mut increases: usize = 0;
    let mut possible_last_depth = None;

    for depth_str in INPUT.lines() {
        let next_depth = depth_str.parse::<usize>().map_err(|_| ())?;

        if let Some(last_depth) = possible_last_depth {
            if next_depth > last_depth {
                increases += 1;
            }
        }

        possible_last_depth.replace(next_depth);
    }

    libc_println!("{} increases", increases);

    Ok(())
}
