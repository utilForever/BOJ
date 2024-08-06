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

fn backtrack(tastes: &Vec<(i64, i64)>, ret: &mut i64, idx: usize, selected: Vec<usize>) {
    if idx == tastes.len() {
        if selected.is_empty() {
            return;
        }

        let mut sour = 1;
        let mut bitter = 0;

        for i in 0..selected.len() {
            sour *= tastes[selected[i]].0;
            bitter += tastes[selected[i]].1;
        }

        *ret = (*ret).min((sour - bitter).abs());
        return;
    }

    let mut selected_new = selected.clone();
    selected_new.push(idx);

    backtrack(tastes, ret, idx + 1, selected_new);
    backtrack(tastes, ret, idx + 1, selected);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut tastes = vec![(0, 0); n];

    for i in 0..n {
        tastes[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = i64::MAX;

    backtrack(&tastes, &mut ret, 0, Vec::new());

    writeln!(out, "{ret}").unwrap();
}
