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

fn calculate(n: usize, p: usize, v: usize, num_subgroup: usize) -> usize {
    let each_participant = (n as f64).powf(1.0 / num_subgroup as f64) as usize;
    let mut rest_participants = 0;

    while (each_participant + 1).pow(rest_participants)
        * each_participant.pow(num_subgroup as u32 - rest_participants)
        < n
    {
        rest_participants += 1;
    }

    let total_participants = num_subgroup * each_participant + rest_participants as usize;
    total_participants * p + num_subgroup * v
}

// Reference: http://www.math.bas.bg/infos/files/2011-06-25-sol-A7.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, p, v) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    if n == 1 {
        writeln!(out, "0").unwrap();
    } else {
        let mut ans = calculate(n, p, v, 1);
        let mut num_subgroup = 2;

        while 2_usize.pow(num_subgroup) <= n {
            let ret = calculate(n, p, v, num_subgroup as usize);
            if ret < ans {
                ans = ret;
            }

            num_subgroup += 1;
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
