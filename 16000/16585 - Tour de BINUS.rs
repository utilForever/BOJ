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
    let mut rooms = vec![0; n];

    for i in 0..n {
        rooms[i] = scan.token::<i64>();
    }

    let (x1, d1) = (scan.token::<usize>() - 1, scan.token::<String>());
    let (x2, d2) = (scan.token::<usize>() - 1, scan.token::<String>());

    let mut ret_ayu = 0;
    let mut ret_budi = 0;

    if d1 == "left" {
        for i in (0..=x1).rev() {
            ret_ayu += rooms[i];
        }
    } else {
        for i in x1..n {
            ret_ayu += rooms[i];
        }
    }

    if d2 == "left" {
        for i in (0..=x2).rev() {
            ret_budi += if rooms[i] == 0 { 1 } else { 0 };
        }
    } else {
        for i in x2..n {
            ret_budi += if rooms[i] == 0 { 1 } else { 0 };
        }
    }

    writeln!(out, "{ret_ayu} {ret_budi}").unwrap();
}
