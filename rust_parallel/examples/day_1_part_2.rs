use anyhow::{Result, Error};
use tokio::{
    fs::File,
    io::{self, AsyncBufRead, AsyncBufReadExt, BufReader, Lines},
};

const WINDOW_SIZE: usize = 4;

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/../inputs/day_1.csv")).await?;
    let file_reader = BufReader::new(file);
    let lines = file_reader.lines();
    let mut lines_windows =
        LinesWindows::<BufReader<File>, usize, Box<dyn Fn(String) -> Result<usize>>, WINDOW_SIZE>::new(
            lines,
            Box::new(|line: String| line.parse::<usize>().map_err(Error::msg)),
        )
        .await?;

    let mut increases: usize = 0;

    loop {
        match lines_windows.next_window().await? {
            Some(window) => {
                let window_part_1: usize = window[1..].iter().sum();
                let window_part_2: usize = window[..(WINDOW_SIZE - 1)].iter().sum();

                if window_part_2 > window_part_1 {
                    increases += 1;
                }
            }
            None => {
                break;
            }
        }
    }

    println!("{} increases", increases);

    Ok(())
}

struct LinesWindows<
    R: AsyncBufRead + Unpin,
    V: Copy + Default,
    U: Fn(String) -> Result<V>,
    const T: usize,
> {
    lines: Lines<R>,
    window: [V; T],
    index: usize,
    next_line: Option<V>,
    transform: U,
    done: bool,
}

impl<R: AsyncBufRead + Unpin, V: Copy + Default, U: Fn(String) -> Result<V>, const T: usize>
    LinesWindows<R, V, U, T>
{
    pub async fn new(lines: Lines<R>, transform: U) -> Result<Self> {
        let mut lines_windows = Self {
            lines,
            window: [Default::default(); T],
            index: 0,
            next_line: None,
            transform,
            done: false,
        };

        for _ in 0..T {
            let _ = lines_windows.next_window().await?;
        }

        Ok(lines_windows)
    }

    async fn next_line(&mut self) -> io::Result<Option<(usize, String)>> {
        if let Some(line) = self.lines.next_line().await? {
            let result = io::Result::Ok(Some((self.index, line)));
            self.index += 1;
            result
        } else {
            Ok(None)
        }
    }

    pub async fn next_window(&mut self) -> Result<Option<&[V; T]>> {
        if let Some(line) = self.next_line.take() {
            self.window.rotate_right(1);
            self.window[0] = line;
        }

        if let Some((_, line)) = self.next_line().await? {
            self.next_line = Some((self.transform)(line)?);
            Ok(Some(&self.window))
        } else if !self.done {
            self.done = true;
            Ok(Some(&self.window))
        } else {
            Ok(None)
        }
    }
}
