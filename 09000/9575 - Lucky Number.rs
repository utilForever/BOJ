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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut a = vec![0; n];

        for i in 0..n {
            a[i] = scan.token::<i64>();
        }

        let m = scan.token::<usize>();
        let mut b = vec![0; m];

        for i in 0..m {
            b[i] = scan.token::<i64>();
        }

        let k = scan.token::<usize>();
        let mut c = vec![0; k];

        for i in 0..k {
            c[i] = scan.token::<i64>();
        }

        let mut ret = HashSet::new();

        for i in 0..n {
            for j in 0..m {
                for l in 0..k {
                    let sum = a[i] + b[j] + c[l];
                    let mut num = sum;
                    let mut check = true;

                    while num > 0 {
                        if num % 10 != 5 && num % 10 != 8 {
                            check = false;
                            break;
                        }

                        num /= 10;
                    }

                    if check {
                        ret.insert(sum);
                    }
                }
            }
        }

        writeln!(out, "{}", ret.len()).unwrap();
    }
}
