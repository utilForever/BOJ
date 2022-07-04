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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut cnt = vec![vec![0; 4]; m];

    for _ in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            match c {
                'A' => cnt[j][0] += 1,
                'C' => cnt[j][1] += 1,
                'G' => cnt[j][2] += 1,
                'T' => cnt[j][3] += 1,
                _ => (),
            }
        }
    }

    let mut ret_string = String::new();
    let mut ret_val = 0;

    for i in 0..m {
        let mut max_val = 0;
        let mut max_idx = 0;

        for j in 0..4 {
            if cnt[i][j] > max_val {
                max_val = cnt[i][j];
                max_idx = j;
            }
        }

        match max_idx {
            0 => ret_string.push('A'),
            1 => ret_string.push('C'),
            2 => ret_string.push('G'),
            3 => ret_string.push('T'),
            _ => (),
        }

        ret_val += n - max_val as i64;
    }

    writeln!(out, "{}", ret_string).unwrap();
    writeln!(out, "{}", ret_val).unwrap();
}
