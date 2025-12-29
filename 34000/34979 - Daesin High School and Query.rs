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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut displeasures = vec![vec![0; n]; 4];

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (a, b) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

            displeasures[a][b] += 1;

            if a > 0 {
                displeasures[a - 1][b] += 1;
            }

            if a < 3 {
                displeasures[a + 1][b] += 1;
            }

            if b > 0 {
                displeasures[a][b - 1] += 1;
            }

            if b < n - 1 {
                displeasures[a][b + 1] += 1;
            }
        } else {
            let a = scan.token::<usize>() - 1;
            let mut ret_idx = 0;
            let mut ret_val = displeasures[a][0];

            for i in 1..n {
                if displeasures[a][i] > ret_val {
                    ret_val = displeasures[a][i];
                    ret_idx = i;
                }
            }

            writeln!(out, "{}", ret_idx + 1).unwrap();
        }
    }

    let mut ret_val = displeasures[0][0];
    let mut ret_floor = 0;
    let mut ret_class = 0;

    for i in 0..4 {
        for j in 0..n {
            if displeasures[i][j] > ret_val
                || (displeasures[i][j] == ret_val
                    && (i < ret_floor || (i == ret_floor && j < ret_class)))
            {
                ret_val = displeasures[i][j];
                ret_floor = i;
                ret_class = j;
            }
        }
    }

    writeln!(out, "{} {}", ret_floor + 1, ret_class + 1).unwrap();
}
