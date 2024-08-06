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

// Reference: http://publications.waset.org/9996770/pdf
fn calculate_phi(mut val: usize) -> usize {
    if val == 1 {
        return 1;
    }

    let mut ret = 1;
    let mut i = 2;

    while i * i <= val {
        while val % i == 0 {
            val /= i;

            if val % i == 0 {
                ret *= i;
            } else {
                ret *= i - 1;
            }
        }

        i += 1;
    }

    if val != 1 {
        ret *= val - 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut ret = 1;

        for _ in 0..n {
            let x = scan.token::<usize>();
            ret = (ret * calculate_phi(x)) % 1_000_000_007;
        }

        writeln!(out, "{}", ret).unwrap();
    }
}
