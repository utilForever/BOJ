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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut railroads = vec![0; n + 1];
    let mut s = scan.token::<usize>();

    for _ in 0..m - 1 {
        let e = scan.token::<usize>();

        if s < e {
            railroads[s] += 1;
            railroads[e] -= 1;
        } else {
            railroads[e] += 1;
            railroads[s] -= 1;
        }

        s = e;
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + railroads[i];
    }

    let mut information = vec![vec![0; 3]; n];

    for i in 1..n {
        information[i][0] = scan.token::<i64>();
        information[i][1] = scan.token::<i64>();
        information[i][2] = scan.token::<i64>();
    }

    let mut ret = 0;

    for i in 1..n {
        ret += (information[i][0] * prefix_sum[i])
            .min(information[i][1] * prefix_sum[i] + information[i][2]);
    }

    writeln!(out, "{ret}").unwrap();
}
