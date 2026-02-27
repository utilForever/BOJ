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

    let phases = [
        "New", "Crescent", "Crescent", "Crescent", "Crescent", "Quarter", "Quarter", "Quarter",
        "Quarter", "Gibbous", "Gibbous", "Gibbous", "Gibbous", "Gibbous", "Full", "Gibbous",
        "Gibbous", "Gibbous", "Gibbous", "Gibbous", "Quarter", "Quarter", "Quarter", "Crescent",
        "Crescent", "Crescent", "Crescent", "Crescent",
    ];

    let n = scan.token::<usize>();
    let mut observations = vec![String::new(); n];

    for i in 0..n {
        observations[i] = scan.token::<String>();
    }

    for d in 1..=28 {
        let mut check = true;
        let mut idx = d % 28;

        for observation in observations.iter() {
            if observation != &phases[idx] {
                check = false;
                break;
            }

            idx = (idx + d) % 28;
        }
        

        if check {
            writeln!(out, "{d}").unwrap();
            return;
        }
    }
}
