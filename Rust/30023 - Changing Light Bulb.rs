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
    let mut colors = vec![0; n];

    let s = scan.token::<String>();

    for (idx, ch) in s.chars().enumerate() {
        colors[idx] = match ch {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        }
    }

    let mut ret = i64::MAX;

    for color in 0..3 {
        let mut colors_new = colors.clone();
        let mut cnt = 0;

        for i in 0..n - 2 {
            if colors_new[i] == color {
                continue;
            }

            let cnt_switch = (color - colors_new[i] + 3) % 3;
            cnt += cnt_switch;

            colors_new[i] = color;
            colors_new[i + 1] = (colors_new[i + 1] + cnt_switch) % 3;
            colors_new[i + 2] = (colors_new[i + 2] + cnt_switch) % 3;
        }

        if colors_new[n - 2] == color && colors_new[n - 1] == color {
            ret = ret.min(cnt);
        }
    }

    writeln!(out, "{}", if ret == i64::MAX { -1 } else { ret }).unwrap();
}
