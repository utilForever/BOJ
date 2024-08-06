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

    let n = scan.token::<i64>();

    for i in 0..n {
        let (usa_gold, usa_silver, usa_bronze) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let (russia_gold, russia_silver, russia_bronze) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let is_usa_win_count =
            usa_gold + usa_silver + usa_bronze > russia_gold + russia_silver + russia_bronze;
        let is_usa_win_color = usa_gold > russia_gold
            || (usa_gold == russia_gold && usa_silver > russia_silver)
            || (usa_gold == russia_gold
                && usa_silver == russia_silver
                && usa_bronze > russia_bronze);

        writeln!(
            out,
            "{} {} {} {} {} {}",
            usa_gold, usa_silver, usa_bronze, russia_gold, russia_silver, russia_bronze
        )
        .unwrap();
        writeln!(
            out,
            "{}",
            if is_usa_win_count && is_usa_win_color {
                "both"
            } else if is_usa_win_count {
                "count"
            } else if is_usa_win_color {
                "color"
            } else {
                "none"
            }
        )
        .unwrap();

        if i != n - 1 {
            writeln!(out).unwrap();
        }
    }
}
