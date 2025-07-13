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

    let k = scan.token::<i64>();

    for t in 1..=k {
        writeln!(out, "Data Set {t}:").unwrap();

        let n = scan.token::<usize>();
        let mut species = vec![0; n];
        let mut eats = vec![vec![0; n]; n];

        for i in 0..n {
            species[i] = scan.token::<i64>();

            for j in 0..i {
                eats[i][j] = scan.token::<i64>();
            }
        }

        let mut ret = species.clone();

        for i in 1..n {
            let mut remain = ret[i];

            for j in 0..i {
                let need = eats[i][j];

                if need == 0 {
                    continue;
                }

                remain = remain.min(ret[j] / need);
            }

            for j in 0..i {
                ret[j] -= remain * eats[i][j];
            }

            ret[i] = remain;
        }

        for i in 0..n {
            writeln!(out, "{}", ret[i]).unwrap();
        }

        if t != k {
            writeln!(out).unwrap();
        }
    }
}
