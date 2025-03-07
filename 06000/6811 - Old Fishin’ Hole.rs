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

    let (point_trout, point_pike, point_pickerel, point_total) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = 0;

    for trout in 0..=point_total / point_trout {
        for pike in 0..=point_total / point_pike {
            for pickerel in 0..=point_total / point_pickerel {
                if trout == 0 && pike == 0 && pickerel == 0 {
                    continue;
                }

                if trout * point_trout + pike * point_pike + pickerel * point_pickerel
                    <= point_total
                {
                    writeln!(
                        out,
                        "{trout} Brown Trout, {pike} Northern Pike, {pickerel} Yellow Pickerel"
                    )
                    .unwrap();
                    ret += 1;
                }
            }
        }
    }

    writeln!(out, "Number of ways to catch fish: {ret}").unwrap();
}
