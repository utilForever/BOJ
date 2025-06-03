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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, l, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut pos_nc = Vec::new();

    for (idx, &c) in s.iter().enumerate() {
        if c == 'N' || c == 'C' {
            pos_nc.push(idx);
        }
    }

    if pos_nc.len() < 2 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut ret = 0;

    for i in 0..pos_nc.len() - 1 {
        let (x, y) = (pos_nc[i], pos_nc[i + 1]);

        if s[x] != 'N' || s[y] != 'C' {
            continue;
        }

        if (x + y) % 2 == 1 {
            continue;
        }

        if s[(x + y) / 2] != 'P' {
            continue;
        }

        let prev = if i == 0 { usize::MAX } else { pos_nc[i - 1] };
        let next = if i + 2 < pos_nc.len() {
            pos_nc[i + 2]
        } else {
            n
        };

        let a = prev.wrapping_add(1);
        let b = x;
        let c = y + 1;
        let d = next;

        for j in a..=b {
            let left = c.max(j + l);
            let right = d.min(j + r);

            if left > d {
                continue;
            }

            if left <= right {
                ret += right - left + 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
