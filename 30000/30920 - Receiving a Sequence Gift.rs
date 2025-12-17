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

    let n = scan.token::<usize>();
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];
    let mut c = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<usize>();
        b[a[i]] = i;
    }

    let mut ret = Vec::new();

    for i in 1..=n {
        if c[i] == 0 {
            if a[i] == i {
                continue;
            }

            ret.push(b[i]);

            let mut idx = i;

            loop {
                ret.push(idx);
                c[a[idx]] = 1;

                if idx == b[i] {
                    break;
                }

                idx = a[idx];
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        writeln!(out, "{val} 1 {n}").unwrap();
    }
}
