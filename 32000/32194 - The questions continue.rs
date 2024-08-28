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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut yes = vec![0; n + 2];
    let mut no = vec![0; n + 2];

    yes[1] = 1;

    for i in 2..=n + 1 {
        let (command, x, y) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if command == 1 {
            if yes[y] - yes[x - 1] == y - x + 1 {
                writeln!(out, "Yes").unwrap();
                yes[i] = yes[i - 1] + 1;
                no[i] = no[i - 1];
            } else {
                writeln!(out, "No").unwrap();
                yes[i] = yes[i - 1];
                no[i] = no[i - 1] + 1;
            }
        } else {
            if no[y] - no[x - 1] == y - x + 1 {
                writeln!(out, "Yes").unwrap();
                yes[i] = yes[i - 1] + 1;
                no[i] = no[i - 1];
            } else {
                writeln!(out, "No").unwrap();
                yes[i] = yes[i - 1];
                no[i] = no[i - 1] + 1;
            }
        }
    }
}
