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

struct Actor {
    pub parent: Option<String>,
    pub position: (i64, i64),
    pub size: (i64, i64),
    pub color: i64,
    pub parent_origin: (i64, i64),
    pub anchor_point: (i64, i64),
    pub screen_position: (i64, i64),
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (w, h) = (scan.token::<i64>(), scan.token::<i64>());
    let q = scan.token::<i64>();

    for _ in 0..q {
        let command = scan.token::<String>();z

        match command.as_str() {
            "New" => {

            },
            "Add" => {

            },
            "Remove" => {

            },
            "Unparent" => {

            },
            "SetProperty" => {

            },
            "GetProperty" => {

            },
            _ => panic!("Unknown command: {}", command),
        }
    }
}
