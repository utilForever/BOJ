use io::Write;
use std::{collections::HashSet, io, str};

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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut teams = vec![(String::new(), String::new()); n];

    for i in 0..n {
        let (name, institution) = (scan.token::<String>(), scan.token::<String>());
        teams[i] = (name, institution);
    }

    let mut set_institution = HashSet::new();

    for i in 0..m {
        set_institution.insert(teams[i].1.clone());
    }

    let mut ret = Vec::with_capacity(k);

    for i in m..n {
        if set_institution.contains(&teams[i].1) {
            continue;
        }

        ret.push(teams[i].0.clone());
        set_institution.insert(teams[i].1.clone());

        if ret.len() == k {
            break;
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for name in ret {
        writeln!(out, "{name}").unwrap();
    }
}
