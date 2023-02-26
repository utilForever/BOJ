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

    let mut records = [(0.0, ' '); 8];
    let scores = [10, 8, 6, 5, 4, 3, 2, 1];

    for i in 0..8 {
        let (time, team) = (scan.token::<String>(), scan.token::<String>());
        let time = time.split(':').collect::<Vec<_>>();
        let time = time[0].parse::<f64>().unwrap() * 60.0 * 1000.0
            + time[1].parse::<f64>().unwrap() * 1000.0
            + time[2].parse::<f64>().unwrap();

        records[i] = (time, team.chars().next().unwrap());
    }

    records.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut total_red = 0;
    let mut total_blue = 0;

    for (idx, record) in records.iter().enumerate() {
        if record.1 == 'R' {
            total_red += scores[idx];
        } else {
            total_blue += scores[idx];
        }
    }

    writeln!(
        out,
        "{}",
        if total_red > total_blue {
            "Red"
        } else {
            "Blue"
        }
    )
    .unwrap();
}
