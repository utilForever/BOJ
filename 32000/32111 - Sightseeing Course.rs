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

    let n = scan.token::<usize>();
    let mut check = scan.token::<String>().chars().collect::<Vec<_>>();

    if check.iter().all(|&x| x == 'O') {
        writeln!(out, "YES").unwrap();
        writeln!(out, "{}", "+".repeat(n)).unwrap();
        return;
    }

    if check.iter().all(|&x| x == 'X') {
        writeln!(out, "YES").unwrap();
        writeln!(out, "{}", "-".repeat(n)).unwrap();
        return;
    }

    check.extend(check.clone());

    let mut positions_o = Vec::new();
    let mut ret = vec![' '; n];

    for i in 0..2 * n {
        if check[i] == 'O' {
            positions_o.push(i);
        }

        ret[i % n] = '+';
    }

    for i in 1..positions_o.len() {
        let diff = positions_o[i] - positions_o[i - 1] - 1;

        if diff > 0 && diff % 2 == 0 {
            writeln!(out, "NO").unwrap();
            return;
        }

        for j in 0..diff {
            ret[(positions_o[i - 1] + j + 1) % n] = if j < diff / 2 { '+' } else { '-' };
        }
    }

    writeln!(out, "YES").unwrap();

    for i in 0..n {
        write!(out, "{}", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
