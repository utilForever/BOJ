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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>();
    }

    let mut check = true;

    nums.windows(2).for_each(|w| {
        if w[0] > w[1] {
            check = false;
        }
    });

    if check {
        writeln!(out, "YES").unwrap();
        return;
    }

    let mut is_prime = vec![true; 1_000_001];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= 1_000_000 {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=1_000_000).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    let mut check = false;

    for i in 0..n - 1 {
        check |= nums[i] != 1 && !is_prime[nums[i]];
        check |= nums[i] != 1 && nums[i + 1] != 1;
    }

    check |= nums[n - 1] != 1 && !is_prime[nums[n - 1]];

    writeln!(out, "{}", if check { "YES" } else { "NO" }).unwrap();
}
