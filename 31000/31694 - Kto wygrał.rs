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

    let mut scores_algosia = [0; 18];
    let mut scores_bajtek = [0; 18];

    for i in 0..18 {
        scores_algosia[i] = scan.token::<i64>();
    }

    for i in 0..18 {
        scores_bajtek[i] = scan.token::<i64>();
    }

    let sum_algosia = scores_algosia.iter().sum::<i64>();
    let sum_bajtek = scores_bajtek.iter().sum::<i64>();

    if sum_algosia > sum_bajtek {
        writeln!(out, "Algosia").unwrap();
        return;
    } else if sum_algosia < sum_bajtek {
        writeln!(out, "Bajtek").unwrap();
        return;
    }

    scores_algosia.sort_by(|a, b| b.cmp(a));
    scores_bajtek.sort_by(|a, b| b.cmp(a));

    let mut ret = None;

    for i in 0..18 {
        if scores_algosia[i] > scores_bajtek[i] {
            ret = Some("Algosia");
            break;
        } else if scores_algosia[i] < scores_bajtek[i] {
            ret = Some("Bajtek");
            break;
        }
    }

    writeln!(
        out,
        "{}",
        match ret {
            Some(x) => x,
            None => "remis",
        }
    )
    .unwrap();
}
