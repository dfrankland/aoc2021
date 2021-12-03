use anyhow::{bail, Result};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/../inputs/day_2.csv")).await?;
    let file_reader = BufReader::new(file);
    let mut lines = file_reader.lines();

    let mut horizontal_position: usize = 0;
    let mut depth: usize = 0;

    while let Some(command_str) = lines.next_line().await? {
        if let Some((command, unit_str)) = command_str.split_once(" ") {
            let unit = unit_str.parse::<usize>()?;
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
                _ => bail!("Unknown command"),
            }
        } else {
            bail!("Missing command or unit");
        }
    }

    println!(
        "{} horizontal position and depth",
        horizontal_position * depth
    );

    Ok(())
}
