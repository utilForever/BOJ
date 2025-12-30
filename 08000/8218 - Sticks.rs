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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let k = scan.token::<usize>();
    let mut sticks = Vec::new();

    for color in 1..=k {
        let n = scan.token::<i64>();

        for _ in 0..n {
            let length = scan.token::<i64>();
            sticks.push((length, color));
        }
    }

    sticks.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut ret = vec![0; k + 1];

    for stick in sticks.iter() {
        let mut first = (0, 0);
        let mut second = (0, 0);

        for color in 1..=k {
            if stick.1 == color {
                continue;
            }

            if ret[color] > first.0 {
                second = first;
                first = (ret[color], color);
            } else if ret[color] > second.0 {
                second = (ret[color], color);
            }
        }

        ret[stick.1] = ret[stick.1].max(stick.0);

        if second.0 == 0 {
            continue;
        }

        if first.0 + second.0 > stick.0 {
            writeln!(
                out,
                "{} {} {} {} {} {}",
                first.1, first.0, second.1, second.0, stick.1, stick.0
            )
            .unwrap();
            return;
        }
    }

    writeln!(out, "NIE").unwrap();
}
