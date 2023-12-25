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

fn differentiate(f: &Vec<i64>) -> Vec<i64> {
    if f.len() <= 1 {
        vec![0]
    } else {
        let mut ret = Vec::new();

        for i in 0..f.len() - 1 {
            ret.push((i + 1) as i64 * f[i + 1]);
        }

        ret
    }
}

fn is_differentiated_root_exist(f: &Vec<i64>) -> bool {
    let f_diff = differentiate(f);
    let degree = f_diff.len() - 1;

    if degree % 2 != 0 {
        true
    } else if degree == 0 {
        f_diff[0] == 0
    } else {
        f_diff[1] * f_diff[1] - 4 * f_diff[0] * f_diff[2] >= 0
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut f = vec![0; n + 1];
        let mut g = vec![0; m + 1];

        for i in 0..=n {
            f[i] = scan.token::<i64>();
        }

        for i in 0..=m {
            g[i] = scan.token::<i64>();
        }

        f.reverse();
        g.reverse();

        let is_f_diff_root_exist = is_differentiated_root_exist(&f);
        let is_g_diff_root_exist = is_differentiated_root_exist(&g);

        writeln!(
            out,
            "{}",
            if is_f_diff_root_exist && is_g_diff_root_exist {
                // Case 1
                "YES"
            } else if is_f_diff_root_exist ^ is_g_diff_root_exist {
                // Case 2
                "NO"
            } else {
                if n != m {
                    // Case 3
                    "NO"
                } else {
                    if n == 3 {
                        // Case 4
                        if f == g {
                            "YES"
                        } else {
                            "NO"
                        }
                    } else if n == 1 {
                        // Case 5
                        if f[1] == g[1] {
                            "YES"
                        } else {
                            "NO"
                        }
                    } else {
                        // Etc
                        "NO"
                    }
                }
            }
        )
        .unwrap();
    }
}
