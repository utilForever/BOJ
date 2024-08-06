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

    let n = scan.token::<usize>();
    let mut nums = Vec::new();
    let mut zeros = Vec::new();

    for _ in 0..n {
        let num = scan.token::<String>();

        if num.starts_with("0") {
            zeros.push(num);
        } else {
            nums.push(num);
        }
    }

    if nums.is_empty() {
        writeln!(out, "INVALID").unwrap();
        return;
    }

    nums.sort_by(|a, b| {
        let mut str1 = a.clone();
        str1.push_str(&b);

        let mut str2 = b.clone();
        str2.push_str(&a);

        str1.parse::<u32>().unwrap().cmp(&str2.parse::<u32>().unwrap())
    });
    zeros.sort_by(|a, b| {
        let mut str1 = a.clone();
        str1.push_str(&b);

        let mut str2 = b.clone();
        str2.push_str(&a);

        str1.parse::<u32>().unwrap().cmp(&str2.parse::<u32>().unwrap())
    });

    if zeros.is_empty() {
        for num in nums {
            write!(out, "{num}").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        let mut idx = 0;

        for j in 1..nums.len() {
            let mut str1 = nums[j].clone();
            str1.push_str(&zeros[0]);
            str1.push_str(&nums[idx]);

            let mut str2 = nums[idx].clone();
            str2.push_str(&zeros[0]);
            str2.push_str(&nums[j]);

            if str1.parse::<u128>().unwrap() < str2.parse::<u128>().unwrap() {
                idx = j;
            }
        }

        write!(out, "{}", nums[idx]).unwrap();

        for num in zeros {
            write!(out, "{num}").unwrap();
        }

        for (i, num) in nums.iter().enumerate() {
            if i == idx {
                continue;
            }

            write!(out, "{num}").unwrap();
        }

        writeln!(out).unwrap();
    }
}
