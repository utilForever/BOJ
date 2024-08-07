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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    if n == 2 {
        println!("+ +");

        let val_a = scan.token::<i64>();

        println!("+ -");

        let val_b = scan.token::<i64>();

        println!("answer: {} {}", (val_a + val_b) / 2, (val_a - val_b) / 2);

        return;
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                print!("-");
            } else {
                print!("+");
            }

            if j != n - 1 {
                print!(" ");
            }
        }

        println!();

        nums[i] = scan.token::<i64>();
    }

    let val = nums.iter().sum::<i64>() / (n as i64 - 2);
    let mut ret = vec![0; n];

    for i in 0..n {
        ret[i] = (val - nums[i]) / 2;
    }

    print!("answer: ");

    for i in 0..n {
        print!("{}", ret[i]);

        if i != n - 1 {
            print!(" ");
        }
    }
}
