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
    let mut nums = vec![String::new(); n];
    let mut priorities = [(0, 0, false); 26];

    for i in 0..26 {
        priorities[i] = (0, i, false);
    }

    for i in 0..n {
        nums[i] = scan.token::<String>();
    }

    for i in 0..n {
        for (idx, c) in nums[i].chars().rev().enumerate() {
            priorities[(c as u8 - b'A') as usize].0 += 10i64.pow(idx as u32);

            if idx == nums[i].len() - 1 {
                priorities[(c as u8 - b'A') as usize].2 = true;
            }
        }
    }

    priorities.sort();

    if priorities[0].0 != 0 && priorities[0].2 {
        let mut idx = 1;

        while idx < 26 && priorities[idx].2 {
            idx += 1;
        }

        let temp = priorities[idx];

        for i in (0..idx).rev() {
            priorities[i + 1] = priorities[i];
        }

        priorities[0] = temp;
    }

    let mut converted = [0; 26];
    let mut num = 9;

    for i in (0..26).rev() {
        if priorities[i].0 == 0 {
            break;
        }

        converted[priorities[i].1] = num;
        num -= 1;
    }

    let mut ret = 0i64;

    for num in nums {
        let mut val = 0;

        for c in num.chars() {
            val = val * 10 + converted[(c as u8 - b'A') as usize];
        }

        ret += val;
    }

    writeln!(out, "{ret}").unwrap();
}
