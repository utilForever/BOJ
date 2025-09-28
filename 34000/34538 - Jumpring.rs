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

fn check(s: &[u8], u: &[u8]) -> bool {
    let (n, m) = (s.len(), u.len());

    if n < m || n - m > (n + 1) / 2 {
        return false;
    }

    let mut keep = vec![0; m + 1];
    let mut delete = vec![0; m + 1];

    keep[0] = 1;

    let mut keep_next = vec![0; m + 1];
    let mut delete_next = vec![0; m + 1];

    for &c in s {
        keep_next.fill(0);
        delete_next.clone_from_slice(&keep);

        for i in 0..m {
            if u[i] == c && (keep[i] != 0 || delete[i] != 0) {
                keep_next[i + 1] = 1;
            }
        }

        std::mem::swap(&mut keep, &mut keep_next);
        std::mem::swap(&mut delete, &mut delete_next);
    }

    keep[m] != 0 || delete[m] != 0
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (_, _) = (scan.token::<usize>(), scan.token::<usize>());
        let s = scan.token::<String>();
        let u = scan.token::<String>();

        writeln!(
            out,
            "{}",
            if check(&s.as_bytes(), &u.as_bytes()) {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }
}
