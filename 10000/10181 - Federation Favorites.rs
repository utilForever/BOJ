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

    loop {
        let n = scan.token::<i64>();

        if n == -1 {
            break;
        }

        let mut aliquots = vec![0; n as usize];
        let mut sum = 0;
        let mut num_aliquots = 0;

        for i in 1..n {
            if n % i == 0 {
                aliquots[num_aliquots] = i;
                sum += i;
                num_aliquots += 1;
            }
        }

        if sum != n {
            writeln!(out, "{n} is NOT perfect.").unwrap();
        } else {
            write!(out, "{n} = ").unwrap();

            for i in 0..num_aliquots - 1 {
                write!(out, "{} + ", aliquots[i]).unwrap();
            }

            writeln!(out, "{}", aliquots[num_aliquots - 1]).unwrap();
        }
    }
}
