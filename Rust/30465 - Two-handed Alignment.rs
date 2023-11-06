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
    let mut nums = vec![0; n + 1];
    let mut visited = vec![false; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    if n % 2 == 1 && nums[n / 2 + 1] != (n / 2 + 1) as i64 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut cnt_left_side = 0_i64;
    let mut cnt_right_side = 0_i64;
    let mut ret = 0;

    for i in 1..=n {
        if visited[i] || nums[i] == i as i64 {
            continue;
        }

        let mut idx = i;
        let mut cnt = 0;
        let is_left_side = i <= n / 2;
        let mut is_same_side = true;

        while !visited[idx] {
            visited[idx] = true;
            idx = nums[idx] as usize;
            cnt += 1;

            if is_left_side != (idx <= n / 2) {
                is_same_side = false;
            }
        }

        if is_same_side {
            if is_left_side {
                cnt_left_side += 1;
            } else {
                cnt_right_side += 1;
            }
        }

        ret += cnt - 1;
    }

    writeln!(
        out,
        "{}",
        ret + cnt_left_side + cnt_right_side + (cnt_left_side - cnt_right_side).abs()
    )
    .unwrap();
}
