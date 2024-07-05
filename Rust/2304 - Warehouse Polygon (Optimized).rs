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
    let mut pillars = vec![0; 1001];
    let mut height_max = 0;

    for _ in 0..n {
        let (w, h) = (scan.token::<usize>(), scan.token::<usize>());

        pillars[w] = h;
        height_max = height_max.max(h);
    }

    let mut height_curr = 0;
    let mut height_max_left = 0;
    let mut height_max_right = 0;
    let mut ret = 0;

    for i in 1..=1000 {
        if pillars[i] == height_max {
            height_max_left = i;
            break;
        }

        if pillars[i] > height_curr {
            height_curr = pillars[i];
        }

        ret += height_curr;
    }

    height_curr = 0;

    for i in (1..=1000).rev() {
        if pillars[i] == height_max {
            height_max_right = i;
            break;
        }

        if pillars[i] > height_curr {
            height_curr = pillars[i];
        }

        ret += height_curr;
    }

    ret += height_max * (height_max_right - height_max_left + 1);

    writeln!(out, "{ret}").unwrap();
}
