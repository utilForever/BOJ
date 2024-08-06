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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<i64>();
    let mut left = 1;
    let mut right = n - 1;
    let mut pos_start = n;
    let mut dist = 0;
    let mut direction = 1;

    while left <= right {
        let mid = (left + right) / 2;
        dist += direction * mid;
        pos_start = pos_start.min(n - dist);

        left = mid + 1;
        direction *= -1;
    }

    left = 1;
    right = n - 1;
    pos_start = pos_start.abs();
    direction = 1;

    let mut query = |idx| -> bool {
        println!("? {}", idx);
        let val = scan.token::<i64>();
        val == 1
    };

    let mut ret = n;

    query(pos_start);

    while left <= right {
        let mid = (left + right) / 2;
        pos_start += direction * mid;

        if query(pos_start) {
            ret = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }

        direction *= -1;
    }

    println!("= {}", ret);
}
