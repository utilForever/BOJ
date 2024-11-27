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

    let (x, q) = (scan.token::<i64>(), scan.token::<i64>());
    let mut salaries = Vec::with_capacity(40000);
    salaries.push(x);

    let multiplier;
    let mut idx = 1;

    loop {
        salaries.push(salaries[idx - 1] + 1);

        if salaries[idx] % idx as i64 != 0 {
            salaries[idx] += idx as i64 - (salaries[idx] % idx as i64);
        }

        if idx as i64 * idx as i64 == salaries[idx] {
            multiplier = idx as i64;
            break;
        }

        idx += 1;
    }

    for _ in 0..q {
        let a = scan.token::<usize>();

        writeln!(
            out,
            "{}",
            if a < salaries.len() {
                salaries[a]
            } else {
                multiplier * a as i64
            }
        )
        .unwrap();
    }
}
