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

// Reference: https://upload.acmicpc.net/a7f700a1-d2e8-4e42-bd6a-527ece5f3399/
// Reference: https://leeh18.tistory.com/5
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (s, t) = (scan.token::<i64>(), scan.token::<i64>());

    let calculate = |val: i64| -> i64 {
        // Precompute a list of val / i
        let mut list = Vec::new();
        let mut idx = 1;

        while idx * idx <= val {
            if list.last().is_none() || *list.last().unwrap() != val / idx {
                list.push(val / idx);
            }

            idx += 1;
        }

        for i in 1..idx {
            list.push(i);
        }

        list.sort();
        list.dedup();

        let mut ret = 0;

        for i in 0..list.len() {
            if i > 0 && list[i] % 2 == list[i - 1] % 2 {
                continue;
            }

            ret += if list[i] % 2 == 0 { 1 } else { -1 } * (val / list[i]);
        }

        ret
    };

    writeln!(out, "{}", calculate(t) - calculate(s - 1)).unwrap();
}
