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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut left = 0_i64;
        let mut right = 10_i64.pow(18);
        let mut is_found = false;
        let mut ret = 0;

        while left < right {
            let mut b = 2_i64.pow(30);
            let mut val = 0_i64;

            while b > 0 {
                if left + (val + b).pow(2) <= right {
                    println!("? {left} {}", val + b);

                    let input = scan.token::<String>();

                    if input == "+" {
                        val += b;
                    } else if input == "0" {
                        is_found = true;
                        ret = left + (val + b).pow(2);
                        break;
                    }
                }

                b /= 2;
            }

            if is_found {
                break;
            }

            right = left + (val + 1).pow(2) - 1;
            left = left + val.pow(2);
        }

        println!("! {ret}");
    }
}
