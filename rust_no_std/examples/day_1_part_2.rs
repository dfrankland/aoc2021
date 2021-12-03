#![no_std]
#![no_main]

use libc_print::libc_println;
#[allow(unused_imports)]
use rust_no_std::my_panic;

const WINDOW_SIZE: usize = 4;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    main_result::<WINDOW_SIZE>().unwrap();
    0
}

fn main_result<const T: usize>() -> Result<(), ()> {
    const INPUT: &str = include_str!("../../inputs/day_1.csv");

    let mut increases: usize = 0;
    let mut window: [usize; T] = [0; T];

    let mut check_window = |window: &[usize; T]| {
        let window_part_1: usize = window[1..].iter().sum();
        let window_part_2: usize = window[..(T - 1)].iter().sum();

        if window_part_2 > window_part_1 {
            increases += 1;
        }
    };

    let mut lines = INPUT.lines().enumerate();

    loop {
        match lines.next() {
            Some((index, depth_str)) => {
                let next_depth = depth_str.parse::<usize>().map_err(|_| ())?;

                if index >= T {
                    check_window(&window);
                }

                window.rotate_right(1);
                window[0] = next_depth;
            }
            None => {
                check_window(&window);
                break;
            }
        }
    }

    libc_println!("{} increases", increases);

    Ok(())
}
