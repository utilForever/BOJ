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

    loop {
        let f = scan.token::<usize>();

        if f == 0 {
            break;
        }

        let r = scan.token::<usize>();
        let mut fronts = vec![0; f];
        let mut rears = vec![0; r];

        for i in 0..f {
            fronts[i] = scan.token::<i64>();
        }

        for i in 0..r {
            rears[i] = scan.token::<i64>();
        }

        let mut ratios = Vec::new();

        for i in 0..f {
            for j in 0..r {
                ratios.push(rears[j] as f64 / fronts[i] as f64);
            }
        }

        ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

        writeln!(
            out,
            "{:.2}",
            ratios.windows(2).map(|w| w[1] / w[0]).fold(0.0, f64::max)
        )
        .unwrap();
    }
}
