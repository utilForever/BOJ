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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let cnt_k = s.iter().filter(|&&c| c == 'K').count();
    let cnt_s = s.iter().filter(|&&c| c == 'S').count();
    let cnt_a = s.iter().filter(|&&c| c == 'A').count();

    let len_case1 = 3 * cnt_k.min(cnt_s).min(cnt_a);
    let len_case2 = if cnt_k > 0 {
        3 * (cnt_k - 1).min(cnt_s).min(cnt_a) + 1
    } else {
        0
    };
    let len_case3 = if cnt_k > 0 && cnt_s > 0 {
        3 * (cnt_k - 1).min(cnt_s - 1).min(cnt_a) + 2
    } else {
        0
    };

    let len = len_case1.max(len_case2).max(len_case3);
    let mut t = String::new();

    for i in 0..s.len() {
        match i % 3 {
            0 => t.push('K'),
            1 => t.push('S'),
            2 => t.push('A'),
            _ => unreachable!(),
        }
    }

    let t = t.chars().collect::<Vec<_>>();
    let mut idx = len;

    for i in (0..s.len()).rev() {
        if idx == 0 {
            break;
        }
        
        if s[i] == t[idx - 1] {
            idx -= 1;

            if idx == 0 {
                break;
            }
        }
    }

    writeln!(out, "{}", s.len() - (len - idx)).unwrap();
}
