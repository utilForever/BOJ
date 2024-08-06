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

    let n = scan.token::<usize>();
    let mut dishes: Vec<(i64, i64)> = Vec::new();

    for _ in 0..n {
        let dish = scan.token::<i64>();

        if dishes.is_empty() || dishes.last().unwrap().0 != dish {
            dishes.push((dish, 1));
        } else {
            dishes.last_mut().unwrap().1 += 1;
        }
    }

    let mut dishes_merged = Vec::new();

    for (dish, cnt) in dishes {
        if dish == 1 && cnt % 2 == 0 {
            if dishes_merged.is_empty() {
                dishes_merged.push((2, cnt / 2));
            } else {
                dishes_merged.last_mut().unwrap().1 += cnt / 2;
            }
        } else if !dishes_merged.is_empty() && dishes_merged.last().unwrap().0 == dish {
            dishes_merged.last_mut().unwrap().1 += cnt;
        } else {
            dishes_merged.push((dish, cnt));
        }
    }

    let mut ret = 0;

    for i in 0..dishes_merged.len() {
        if dishes_merged[i].0 == 1 {
            ret = ret.max(dishes_merged[i].1 / 2);
        } else {
            let mut val = dishes_merged[i].1;

            if i > 0 {
                val += dishes_merged[i - 1].1 / 2;
            }

            if i + 1 < dishes_merged.len() {
                val += dishes_merged[i + 1].1 / 2;
            }

            ret = ret.max(val);
        }
    }

    if ret == 0 {
        writeln!(out, "1").unwrap();
        return;
    }

    writeln!(out, "{}", 2_i64 << (ret as f64).log2().floor() as i64).unwrap();
}
