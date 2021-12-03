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
    const INPUT: &str = include_str!("../../inputs/day_2.csv");

    let mut horizontal_position: usize = 0;
    let mut depth: usize = 0;

    for command_str in INPUT.lines() {
        let (command, unit_str) = command_str
            .split_once(" ")
            .unwrap_or_else(|| unreachable!("Missing command or unit"));
        let unit = unit_str.parse::<usize>().map_err(|_| ())?;

        match command {
            "forward" => {
                horizontal_position += unit;
            }
            "up" => {
                depth -= unit;
            }
            "down" => {
                depth += unit;
            }
            _ => unreachable!("Unknown command"),
        }
    }

    libc_println!(
        "{} horizontal position and depth",
        horizontal_position * depth
    );

    Ok(())
}
