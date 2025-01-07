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

    let (n, p, q) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut strawberries = vec![0; n];
    let mut shine_muscats = vec![0; n];
    let diff_robot = p - q;
    let mut counts = vec![0; n];
    let mut ret = true;

    for i in 0..n {
        strawberries[i] = scan.token::<i64>();
    }

    for i in 0..n {
        shine_muscats[i] = scan.token::<i64>();
    }

    for i in 0..n {
        let diff_skewer = strawberries[i] - shine_muscats[i];

        if diff_skewer == 0 {
            continue;
        }

        if diff_skewer * diff_robot >= 0 || diff_skewer.abs() % diff_robot.abs() != 0 {
            ret = false;
            break;
        }

        counts[i] = diff_skewer.abs() / diff_robot.abs();
    }

    if counts.iter().sum::<i64>() > 10000 {
        ret = false;
    }

    if ret {
        writeln!(out, "YES").unwrap();

        for val in counts {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}
