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
    let mut water = 0;
    let mut oxygen = 0;
    let mut temperature = -30;

    for _ in 0..n {
        let (parameter, amount) = (scan.token::<String>(), scan.token::<String>());
        let parameter = parameter.as_str();
        let amount = amount[1..].parse::<i64>().unwrap();

        match parameter {
            "ocean" => water += amount,
            "oxygen" => oxygen += amount,
            "temperature" => temperature += amount,
            _ => panic!("Invalid parameter"),
        }
    }

    writeln!(
        out,
        "{}",
        if water >= 9 && oxygen >= 14 && temperature >= 8 {
            "liveable"
        } else {
            "not liveable"
        }
    )
    .unwrap();
}
