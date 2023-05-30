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

    let is_prime = |val: usize| -> bool {
        let mut idx = 2;

        while idx * idx <= val {
            if val % idx == 0 {
                return false;
            }

            idx += 1;
        }

        true
    };

    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];
    let mut idx = n + 1;

    for i in 1..=n {
        if nums[i] != 0 {
            continue;
        }

        while !is_prime(idx) {
            idx += 1;
        }

        println!("? {i} {idx}");
        idx += 1;

        let ret1 = scan.token::<usize>();

        if i == ret1 {
            nums[i] = i;
            continue;
        }

        let mut idx_cycle = ret1;

        loop {
            println!("? {i} {idx}");
            idx += 1;

            let ret2 = scan.token::<usize>();
            nums[idx_cycle] = ret2;

            if ret1 == ret2 {
                break;
            }

            idx_cycle = ret2;
        }
    }

    print!("! ");

    for i in 1..=n {
        print!("{} ", nums[i]);
    }

    println!();
}
