use io::Write;
use std::{collections::HashSet, io, str};

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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut streaks = HashSet::new();
    let mut ret = vec![(0, String::new()); n];

    for i in 0..n {
        let mut streak = 0;
        let mut streak_max = 0;

        for _ in 0..m {
            let record = scan.token::<String>();

            if record == "." {
                streak += 1;
                streak_max = streak_max.max(streak);
            } else {
                streak = 0;
            }
        }

        let name = scan.token::<String>();

        streaks.insert(streak_max);
        ret[i] = (streak_max, name);
    }

    writeln!(out, "{}", streaks.len()).unwrap();

    for (streak, name) in ret {
        writeln!(out, "{streak} {name}").unwrap();
    }
}
