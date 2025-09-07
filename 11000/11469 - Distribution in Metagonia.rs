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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut n = scan.token::<i64>();
        let mut cnt_two = 0;
        let mut ret = Vec::new();

        while n > 0 {
            while n % 2 == 0 {
                n /= 2;
                cnt_two += 1;
            }

            let mut cnt_three = 0;
            let mut temp = 1;

            while temp * 3 <= n {
                temp *= 3;
                cnt_three += 1;
            }

            ret.push((cnt_two, cnt_three));
            n -= temp;
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for &(two, three) in ret.iter() {
            write!(out, "{} ", 2i64.pow(two as u32) * 3i64.pow(three as u32)).unwrap();
        }

        writeln!(out).unwrap();
    }
}
