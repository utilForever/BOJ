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

const MOD: usize = 100_000_000;

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let mut power_of_2 = [0; 8];
    let mut power_of_5 = [0; 8];

    for i in 0..8 {
        power_of_2[i] = 2usize.pow(i as u32);
        power_of_5[i] = 5usize.pow(i as u32);
    }

    let t = scan.token::<i64>();
    let mut query = |a: usize, b: usize| -> bool {
        println!("? {a} {b}");

        let ret = scan.token::<String>();

        ret == "YES"
    };

    for _ in 0..t {
        let mut left = 0;
        let mut right = 8;

        while left + 1 < right {
            let mid = (left + right) / 2;

            if query(MOD, MOD + power_of_2[mid - 1] * power_of_5[7]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        let a = left;

        left = 0;
        right = 8;

        while left + 1 < right {
            let mid = (left + right) / 2;

            if query(MOD, MOD + power_of_2[7] * power_of_5[mid - 1]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        let b = left;
        let cycle_len = power_of_2[a] * power_of_5[b];

        left = 0;
        right = 9;

        while left + 1 < right {
            let mid = (left + right) / 2;

            if query(mid - 1, mid - 1 + cycle_len) {
                right = mid;
            } else {
                left = mid;
            }
        }

        println!("! {}", left + cycle_len);
    }
}
