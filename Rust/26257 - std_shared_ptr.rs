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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut reference_table = vec![0; m + 1];

    for i in 1..=m {
        reference_table[i] = scan.token::<i64>();
    }

    for _ in 0..q {
        let command = scan.token::<String>();

        match command.as_str() {
            "assign" => {
                let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
                reference_table[x] = reference_table[y];
            }
            "swap" => {
                let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
                reference_table.swap(x, y);
            }
            "reset" => {
                let x = scan.token::<usize>();
                reference_table[x] = 0;
            }
            _ => unreachable!(),
        }
    }

    let mut ret = reference_table
        .iter()
        .filter(|&&x| x != 0)
        .collect::<Vec<_>>();
    ret.sort();
    ret.dedup();

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
