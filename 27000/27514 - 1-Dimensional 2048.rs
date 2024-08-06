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
    let mut elems = [0; 101];

    for _ in 0..n {
        let mut num = scan.token::<u64>();
        if num == 0 {
            continue;
        }
        
        let mut bit = 0;

        while num > 1 {
            num >>= 1;
            bit += 1;
        }

        elems[bit] += 1;
    }

    let mut ret = 0;

    for i in 0..100 {
        elems[i + 1] += elems[i] / 2;
        elems[i] %= 2;

        if elems[i] == 1 {
            ret = i;
        }
    }

    writeln!(out, "{}", 2_i64.pow(ret as u32)).unwrap();
}
