use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/../inputs/day_1.csv")).await?;
    let file_reader = BufReader::new(file);
    let mut lines = file_reader.lines();

    let mut increases: usize = 0;
    let mut possible_last_depth = None;

    while let Some(depth_str) = lines.next_line().await? {
        let next_depth = depth_str.parse::<usize>()?;

        if let Some(last_depth) = possible_last_depth {
            if next_depth > last_depth {
                increases += 1;
            }
        }

        possible_last_depth.replace(next_depth);
    }

    println!("{} increases", increases);

    Ok(())
}
