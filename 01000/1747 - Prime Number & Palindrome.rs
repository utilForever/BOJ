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

    let n = scan.token::<i64>();
    
    if n == 1 {
        writeln!(out, "2").unwrap();
        return;
    }

    let mut prime_numbers = vec![0; 1003002];

    for i in 2..=1003001 {
        prime_numbers[i] = i;
    }

    for i in 2..=(1003001 as f64).sqrt() as usize {
        if prime_numbers[i] == 0 {
            continue;
        }

        for j in (i * i..=1003001).step_by(i) {
            prime_numbers[j] = 0;
        }
    }

    let mut ret = n;

    loop {
        if prime_numbers[ret as usize] == 0 {
            ret += 1;
            continue;
        }

        let mut num = ret;
        let mut rev = 0;

        while num > 0 {
            let digit = num % 10;
            rev = rev * 10 + digit;
            num /= 10;
        }

        if rev == ret {
            break;
        }

        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
