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

    let mut t = 1;

    loop {
        let n = scan.token::<i64>();

        if n == 0 {
            break;
        }

        let mut ret = 0.0;

        for _ in 0..n {
            let (chicken, beef, lamb, nasi) = (
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            let price_chicken = chicken * 0.8;
            let price_beef = beef * 1.0;
            let price_lamb = lamb * 1.2;
            let price_nasi = nasi * 0.8;

            let weight_chicken = chicken / 85.0;
            let weight_beef = beef / 85.0;
            let weight_lamb = lamb / 85.0;

            let cost_chicken = weight_chicken * 15.5;
            let cost_beef = weight_beef * 32.0;
            let cost_lamb = weight_lamb * 40.0;
            let cost_nasi = nasi * 0.2;

            ret += (price_chicken + price_beef + price_lamb + price_nasi)
                - (cost_chicken + cost_beef + cost_lamb + cost_nasi);
        }

        writeln!(out, "Case #{t}: RM{:.2}", ret).unwrap();

        t += 1;
    }
}
