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

    let n: usize = scan.token::<usize>();
    let mut signs_plus = vec![1];
    let mut signs_minus = vec![];
    let mut signs = vec![' '; n + 1];
    let mut ret = vec![' '; n + 1];

    // Step 1: Determine relative signs
    for i in 2..=n {
        println!("? 1 * {}", i);

        let s = scan.token::<char>();
        signs[i] = s;

        if s == '+' {
            signs_plus.push(i);
        } else {
            signs_minus.push(i);
        }
    }

    let mut sign_a1 = ' ';

    // Step 2: Determine sign of A_1
    if signs_plus.len() >= 2 {
        let i = signs_plus[0];
        let j = signs_plus[1];

        println!("? {} + {}", i, j);

        let sign_sum = scan.token::<char>();
        sign_a1 = sign_sum;
    } else if signs_minus.len() >= 2 {
        let i = signs_minus[0];
        let j = signs_minus[1];

        println!("? {} + {}", i, j);

        let sum_sign = scan.token::<char>();

        sign_a1 = if sum_sign == '+' { '-' } else { '+' };
    }

    // Step 3: Assign signs to all elements
    ret[1] = sign_a1;

    for i in 2..=n {
        if signs[i] == '+' {
            ret[i] = sign_a1;
        } else if signs[i] == '-' {
            ret[i] = if sign_a1 == '+' { '-' } else { '+' };
        }
    }

    print!("! ");

    for i in 1..=n {
        print!("{} ", ret[i]);
    }

    println!();
}
