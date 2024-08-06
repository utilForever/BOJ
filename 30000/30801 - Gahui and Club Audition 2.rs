use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let notes = scan.token::<String>();
    let notes = notes.chars().collect::<Vec<_>>();
    let inputs = scan.token::<String>();
    let inputs = inputs.chars().collect::<Vec<_>>();

    let mut converted_notes = Vec::new();
    let mut converted_inputs = Vec::new();
    let mut idx = 0;

    while idx < notes.len() {
        let mut direction = if notes[idx] == 'W' {
            Direction::Up
        } else if notes[idx] == 'A' {
            Direction::Left
        } else if notes[idx] == 'S' {
            Direction::Down
        } else if notes[idx] == 'D' {
            Direction::Right
        } else if notes[idx] == 'L' {
            if notes[idx + 1] == 'U' {
                Direction::LeftUp
            } else {
                Direction::LeftDown
            }
        } else {
            if notes[idx + 1] == 'U' {
                Direction::RightUp
            } else {
                Direction::RightDown
            }
        };

        if notes[idx] == 'L' || notes[idx] == 'R' {
            idx += 2;
        } else {
            idx += 1;
        }

        if idx < notes.len() && notes[idx] == '!' {
            direction = match direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                Direction::LeftUp => Direction::RightDown,
                Direction::LeftDown => Direction::RightUp,
                Direction::RightUp => Direction::LeftDown,
                Direction::RightDown => Direction::LeftUp,
            };
            idx += 1;
        }

        converted_notes.push(direction);
    }

    for &input in inputs.iter() {
        let direction = match input {
            'W' | '8' => Direction::Up,
            'S' | '2' => Direction::Down,
            'A' | '4' => Direction::Left,
            'D' | '6' => Direction::Right,
            '7' => Direction::LeftUp,
            '1' => Direction::LeftDown,
            '9' => Direction::RightUp,
            '3' => Direction::RightDown,
            _ => unreachable!(),
        };

        converted_inputs.push(direction);
    }

    if converted_notes.len() > converted_inputs.len() {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut pos = 0;

    for i in 0..converted_inputs.len() {
        if pos == converted_notes.len() || converted_inputs[i] != converted_notes[pos] {
            pos = 0;
        } else {
            pos += 1;
        }
    }

    writeln!(
        out,
        "{}",
        if pos == converted_notes.len() {
            "Yes"
        } else {
            "No"
        }
    )
    .unwrap();
}
