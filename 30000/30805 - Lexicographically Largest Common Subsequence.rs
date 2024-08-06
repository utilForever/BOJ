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
    let mut a = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<u8>();
    }

    let m = scan.token::<usize>();
    let mut b = vec![0; m];

    for i in 0..m {
        b[i] = scan.token::<u8>();
    }

    let mut val = 100;
    let mut idx_a = 0;
    let mut idx_b = 0;
    let mut ret = Vec::new();

    while val > 0 {
        let pos_a = a[idx_a..].iter().position(|&x| x == val);
        let pos_b = b[idx_b..].iter().position(|&x| x == val);

        if pos_a.is_some() && pos_b.is_some() {
            idx_a = pos_a.unwrap() + idx_a + 1;
            idx_b = pos_b.unwrap() + idx_b + 1;

            ret.push(val);
            continue;
        }

        if idx_a == n || idx_b == m {
            break;
        }

        val -= 1;
    }

    if ret.is_empty() {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
