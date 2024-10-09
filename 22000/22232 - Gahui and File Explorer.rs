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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut files = vec![(String::new(), String::new()); n];
    let mut extensions = HashSet::new();

    for i in 0..n {
        let s = scan.token::<String>();
        let (name, ext) = s.split_at(s.find('.').unwrap());
        files[i] = (name.to_string(), ext[1..].to_string());
    }

    for _ in 0..m {
        extensions.insert(scan.token::<String>());
    }

    files.sort_by(|a, b| {
        if a.0 == b.0 {
            let exist_ext_a = extensions.contains(&a.1);
            let exist_ext_b = extensions.contains(&b.1);

            if exist_ext_a ^ exist_ext_b {
                if exist_ext_a {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            } else {
                a.1.cmp(&b.1)
            }
        } else {
            a.0.cmp(&b.0)
        }
    });

    for (name, ext) in files {
        writeln!(out, "{name}.{ext}").unwrap();
    }
}
