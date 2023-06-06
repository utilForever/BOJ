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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut mbti = vec![String::new(); n];

        for i in 0..n {
            mbti[i] = scan.token::<String>();
        }

        if n > 32 {
            writeln!(out, "0").unwrap();
        } else {
            let mut ret = i64::MAX;

            for i in 0..n - 2 {
                for j in i + 1..n - 1 {
                    for k in j + 1..n {
                        let mbti_a = mbti[i].chars().collect::<Vec<_>>();
                        let mbti_b = mbti[j].chars().collect::<Vec<_>>();
                        let mbti_c = mbti[k].chars().collect::<Vec<_>>();
                        let mut dist = 0;

                        for i in 0..4 {
                            dist += if mbti_a[i] != mbti_b[i] { 1 } else { 0 };
                            dist += if mbti_b[i] != mbti_c[i] { 1 } else { 0 };
                            dist += if mbti_c[i] != mbti_a[i] { 1 } else { 0 };
                        }

                        ret = ret.min(dist);
                    }
                }
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
