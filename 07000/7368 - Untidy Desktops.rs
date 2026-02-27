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

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut windows = vec![(0, 0, 0, 0); n];

        for i in 0..n {
            windows[i] = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
        }

        let mut ret = vec![true; n];

        for i in 0..n {
            for j in i + 1..n {
                let r_left = windows[i].0.max(windows[j].0);
                let r_right =
                    (windows[i].0 + windows[i].3 - 1).min(windows[j].0 + windows[j].3 - 1);
                let r_top = windows[i].1.max(windows[j].1);
                let r_bottom =
                    (windows[i].1 + windows[i].2 - 1).min(windows[j].1 + windows[j].2 - 1);

                if r_left <= r_right && r_top <= r_bottom {
                    ret[i] = false;
                    ret[j] = false;
                }
            }
        }

        writeln!(out, "{}", ret.iter().filter(|&&x| !x).count()).unwrap();
    }
}
