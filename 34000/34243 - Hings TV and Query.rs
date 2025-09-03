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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = s.len();
    let q = scan.token::<i64>();

    // +^+ and -^- patterns
    let mut face1 = vec![0; n];
    let mut face2 = vec![0; n];

    for i in 0..n.saturating_sub(2) {
        if s[i] == '+' && s[i + 1] == '^' && s[i + 2] == '+' {
            face1[i] = 1;
        } else if s[i] == '-' && s[i + 1] == '^' && s[i + 2] == '-' {
            face2[i] = 1;
        }
    }

    let mut prefix_sum_face1 = vec![0; n + 1];
    let mut prefix_sum_face2 = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum_face1[i] = prefix_sum_face1[i - 1] + face1[i - 1];
        prefix_sum_face2[i] = prefix_sum_face2[i - 1] + face2[i - 1];
    }

    // -^-^- pattern
    let mut face1_two = vec![0; n];

    for i in 0..n.saturating_sub(4) {
        if s[i] == '-' && s[i + 1] == '^' && s[i + 2] == '-' && s[i + 3] == '^' && s[i + 4] == '-' {
            face1_two[i] = 1;
        }
    }

    let mut prefix_sum_face1_two = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum_face1_two[i] = prefix_sum_face1_two[i - 1] + face1_two[i - 1];
    }

    // +^x+ and +x^+ patterns (x != '+')
    let mut conv1 = vec![0; n];
    let mut conv2 = vec![0; n];

    for i in 0..n.saturating_sub(3) {
        if s[i] == '+' && s[i + 1] == '^' && s[i + 2] != '+' && s[i + 3] == '+' {
            conv1[i] = 1;
        } else if s[i] == '+' && s[i + 1] != '+' && s[i + 2] == '^' && s[i + 3] == '+' {
            conv2[i] = 1;
        }
    }

    let mut prefix_sum_conv1 = vec![0; n + 1];
    let mut prefix_sum_conv2 = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum_conv1[i] = prefix_sum_conv1[i - 1] + conv1[i - 1];
        prefix_sum_conv2[i] = prefix_sum_conv2[i - 1] + conv2[i - 1];
    }

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        let len = r - l + 1;
        let base = if len >= 3 {
            let cnt_face1 = prefix_sum_face1[r - 2] - prefix_sum_face1[l - 1];
            let cnt_face2 = prefix_sum_face2[r - 2] - prefix_sum_face2[l - 1];

            cnt_face1 - cnt_face2
        } else {
            0
        };
        let delta = if len >= 5 && prefix_sum_face1_two[r - 4] - prefix_sum_face1_two[l - 1] > 0 {
            2
        } else if len >= 3 && prefix_sum_face2[r - 2] - prefix_sum_face2[l - 1] > 0 {
            1
        } else if len >= 4
            && ((prefix_sum_conv1[r - 3] - prefix_sum_conv1[l - 1] > 0)
                || (prefix_sum_conv2[r - 3] - prefix_sum_conv2[l - 1] > 0))
        {
            1
        } else {
            0
        };

        writeln!(out, "{}", base + delta).unwrap();
    }
}
