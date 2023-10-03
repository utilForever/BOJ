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

    let database = [
        3, 2, 1, 2, 3, 3, 3, 3, 1, 1, 3, 1, 3, 3, 1, 2, 2, 2, 1, 2, 1, 1, 2, 2, 2, 1,
    ];

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut nums = s
        .iter()
        .map(|c| database[*c as usize - 'A' as usize])
        .collect::<Vec<_>>();

    while nums.len() > 1 {
        let mut nums_new = Vec::new();
        let mut idx = 0;

        while idx + 1 < nums.len() {
            nums_new.push((nums[idx] + nums[idx + 1]) % 10);
            idx += 2;
        }

        if idx + 1 == nums.len() {
            nums_new.push(nums[idx]);
        }

        nums = nums_new;
    }

    writeln!(
        out,
        "{}",
        if nums[0] % 2 == 1 {
            "I'm a winner!"
        } else {
            "You're the winner?"
        }
    )
    .unwrap();
}
