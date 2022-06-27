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

    let k = scan.token::<i64>();
    let mut remain_turns_first = 0;
    let mut remain_turns_second = 0;

    for _ in 0..k {
        let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
        let fence1 = scan.token::<String>();
        let fence2 = scan.token::<String>();

        let mut fence1 = fence1.chars().collect::<Vec<char>>();
        let mut fence2 = fence2.chars().collect::<Vec<char>>();
        let mut bounds = vec![(0_i64, 0_i64); n as usize + 1];

        if fence1[0] == 'D' {
            std::mem::swap(&mut fence1, &mut fence2);
        }

        let mut x = 0;
        let mut y = 0;

        for i in 0..fence1.len() {
            if fence1[i] == 'R' {
                y += 1;
            } else {
                x += 1;

                if fence1[0] == 'D' {
                    bounds[x].0 = y + 1;
                } else {
                    bounds[x].1 = y;
                }
            }
        }

        x = 0;
        y = 0;

        for i in 0..fence2.len() {
            if fence2[i] == 'R' {
                y += 1;
            } else {
                x += 1;

                if fence2[0] == 'D' {
                    bounds[x].0 = y + 1;
                } else {
                    bounds[x].1 = y;
                }
            }
        }

        let mut remain_down = n;
        let mut remain_right = m;

        while remain_down > 1 && remain_right > 1 {
            remain_down -= 1;
            remain_right -= 1;

            remain_right = remain_right.min(bounds[remain_down as usize].1);

            while remain_down > 1 && remain_right < bounds[remain_down as usize].0 {
                remain_down -= 1;
            }
        }

        remain_turns_first += remain_right - 1;
        remain_turns_second += remain_down - 1;
    }

    writeln!(
        out,
        "{}",
        if remain_turns_first > remain_turns_second {
            "First"
        } else {
            "Second"
        }
    )
    .unwrap();
}
