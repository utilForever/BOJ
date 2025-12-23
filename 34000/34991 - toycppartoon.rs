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

fn find_first_pos(s: &[u8], sub: &[u8]) -> Option<usize> {
    if sub.is_empty() || sub.len() > s.len() {
        return None;
    }

    for i in 0..=s.len() - sub.len() {
        if &s[i..i + sub.len()] == sub {
            return Some(i);
        }
    }

    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().as_bytes().to_vec();
    let t = b"toycartoon".to_vec();

    let mut len_p = 0;
    let mut pos_p = 0;

    for i in (1..=s.len().min(t.len())).rev() {
        let prefix = &s[..i];

        if let Some(pos) = find_first_pos(&t, prefix) {
            len_p = i;
            pos_p = pos;
            break;
        }
    }

    if len_p == 0 {
        let mut candidate = Vec::new();

        candidate.extend_from_slice(&t);
        candidate.push(b'_');
        candidate.extend_from_slice(&s);

        if candidate.len() > 20 {
            writeln!(out, "toycartoon").unwrap();
        } else {
            writeln!(out, "{}", String::from_utf8(candidate).unwrap()).unwrap();
        }

        return;
    }

    let x = &t[..pos_p];
    let y = &t[pos_p + len_p..];
    let remain = &s[len_p..];

    let mut overlap = 0;
    let len_max = remain.len().min(y.len());

    for i in 1..=len_max {
        if remain[remain.len() - i..] == y[..i] {
            overlap = i;
        }
    }

    let y_trimmed = &y[overlap..];

    let mut candidate = Vec::new();
    candidate.extend_from_slice(x);
    candidate.extend_from_slice(&s);
    candidate.extend_from_slice(y_trimmed);

    if candidate.len() > 20 {
        writeln!(out, "toycartoon").unwrap();
    } else {
        writeln!(out, "{}", String::from_utf8(candidate).unwrap()).unwrap();
    }
}
