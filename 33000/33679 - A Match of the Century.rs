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

fn calculate_lis(bullets: &Vec<i64>) -> i64 {
    let mut lis = Vec::new();

    for &val in bullets.iter().skip(1) {
        if val > bullets[0] {
            match lis.binary_search(&val) {
                Ok(pos) => lis[pos] = val,
                Err(pos) => {
                    if pos == lis.len() {
                        lis.push(val);
                    } else {
                        lis[pos] = val;
                    }
                }
            }
        }
    }

    1 + lis.len() as i64
}

fn calculate_score_max(bullets: &Vec<i64>) -> i64 {
    let mut ret = 0;

    for i in 0..bullets.len() {
        let mut bullets_local = Vec::with_capacity(bullets.len());

        for j in 0..bullets.len() {
            bullets_local.push(bullets[(i + j) % bullets.len()]);
        }

        let score = calculate_lis(&bullets_local);
        ret = ret.max(score);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut bullets_a = vec![0; n];
    let mut bullets_b = vec![0; n];

    for i in 0..n {
        bullets_a[i] = scan.token::<i64>();
    }

    for i in 0..n {
        bullets_b[i] = scan.token::<i64>();
    }

    let score_a = calculate_score_max(&bullets_a);
    let score_b = calculate_score_max(&bullets_b);

    writeln!(
        out,
        "{}",
        if score_a > score_b {
            "YJ Win!"
        } else if score_a < score_b {
            "HG Win!"
        } else {
            "Both Win!"
        }
    )
    .unwrap();
}
