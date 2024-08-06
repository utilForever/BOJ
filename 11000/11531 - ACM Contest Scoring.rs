use io::Write;
use std::{collections::HashMap, io, str};

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

    let mut hash_map = HashMap::new();
    let mut ret_solved = 0;
    let mut ret_time = 0;

    loop {
        let m = scan.token::<i64>();

        if m == -1 {
            break;
        }

        let (name, result) = (scan.token::<char>(), scan.token::<String>());

        if result == "right" {
            let cnt = match hash_map.iter().find(|(k, _)| k == &&name) {
                Some((_, v)) => *v,
                None => 0,
            };

            ret_solved += 1;
            ret_time += m + cnt * 20;
        } else {
            hash_map.entry(name).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    writeln!(out, "{ret_solved} {ret_time}").unwrap();
}
