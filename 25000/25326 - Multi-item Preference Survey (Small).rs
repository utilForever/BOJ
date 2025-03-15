use io::Write;
use std::{io, str, vec};

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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut students = vec![(String::new(), String::new(), String::new()); n];

    for i in 0..n {
        let (subject, fruit, color) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        students[i] = (subject, fruit, color);
    }

    for _ in 0..m {
        let (subject, fruit, color) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let mut ret = 0;

        for i in 0..n {
            let mut check = true;

            if subject != "-" && subject != students[i].0 {
                check = false;
            }

            if fruit != "-" && fruit != students[i].1 {
                check = false;
            }

            if color != "-" && color != students[i].2 {
                check = false;
            }

            if check {
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
