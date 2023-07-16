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

    let convert = |s: String| -> String {
        let mut ret = s.chars().collect::<Vec<_>>();

        ret = ret
            .iter_mut()
            .map(|c| match *c {
                '6' => '9',
                '9' => '6',
                _ => *c,
            })
            .collect::<Vec<_>>();
        ret.reverse();

        ret.into_iter().collect::<String>()
    };

    let n = scan.token::<usize>();
    let mut nums = vec![String::new(); n];

    for i in 0..n {
        let num = scan.token::<String>();
        nums[i] = convert(num);
    }

    let mut max = String::new();

    for num in nums.iter() {
        if num.len() > max.len() || (num.len() == max.len() && num > &max) {
            max = num.clone();
        }
    }

    nums.push(max);

    nums.sort_by(|a, b| {
        let ab = a.to_string() + b;
        let ba = b.to_string() + a;

        ab.cmp(&ba)
    });

    for num in nums {
        write!(out, "{}", convert(num)).unwrap();
    }
}
