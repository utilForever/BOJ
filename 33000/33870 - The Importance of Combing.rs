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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut periods = vec![0; n];
    let mut dogs = vec![0; m];

    for i in 0..n {
        periods[i] = scan.token::<usize>();
    }

    for i in 0..m {
        dogs[i] = scan.token::<usize>() - 1;
    }

    let mut day_comb = vec![0; n];
    let mut tangled = vec![false; n];

    for day in 1..=m {
        for i in 0..n {
            if !tangled[i] && day - day_comb[i] > periods[i] {
                tangled[i] = true;
            }
        }

        let dog = dogs[day - 1];

        if tangled[dog] && day_comb[dog] == day - 1 {
            tangled[dog] = false;
        }

        day_comb[dog] = day;
    }

    for i in 0..n {
        if !tangled[i] && m + 1 - day_comb[i] > periods[i] {
            tangled[i] = true;
        }
    }

    writeln!(out, "{}", tangled.iter().filter(|&&x| x).count()).unwrap();
}
