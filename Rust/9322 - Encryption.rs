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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut public_key1 = vec![String::new(); n];
        let mut public_key2 = vec![String::new(); n];
        let mut private_key = vec![String::new(); n];

        for i in 0..n {
            public_key1[i] = scan.token::<String>();
        }

        for i in 0..n {
            public_key2[i] = scan.token::<String>();
        }

        for i in 0..n {
            private_key[i] = scan.token::<String>();
        }

        let mut ret = vec![String::new(); n];

        for i in 0..n {
            let pos = public_key1
                .iter()
                .position(|x| x == &public_key2[i])
                .unwrap();
            ret[pos] = private_key[i].clone();
        }

        for i in 0..n {
            write!(out, "{} ", ret[i]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
