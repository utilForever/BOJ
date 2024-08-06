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

    let alphabets = [
        3, 2, 1, 2, 4, 3, 1, 3, 1, 1, 3, 1, 3, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 2, 2, 1,
    ];

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (a, b) = (scan.token::<String>(), scan.token::<String>());
    let (a, b) = (a.chars().collect::<Vec<_>>(), b.chars().collect::<Vec<_>>());
    let mut nums = Vec::new();

    for i in 0..n.min(m) {
        nums.push(alphabets[(a[i] as u8 - b'A') as usize]);
        nums.push(alphabets[(b[i] as u8 - b'A') as usize]);
    }

    if n > m {
        for i in m..n {
            nums.push(alphabets[(a[i] as u8 - b'A') as usize]);
        }
    } else if n < m {
        for i in n..m {
            nums.push(alphabets[(b[i] as u8 - b'A') as usize]);
        }
    }

    loop {
        if nums.len() == 2 {
            break;
        }

        let mut num_new = Vec::new();

        for i in 0..nums.len() - 1 {
            num_new.push((nums[i] + nums[i + 1]) % 10);
        }

        nums = num_new;
    }

    writeln!(out, "{}%", nums[0] * 10 + nums[1]).unwrap();
}
