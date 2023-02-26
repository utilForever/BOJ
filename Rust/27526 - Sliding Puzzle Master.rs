use io::Write;
use std::{
    io::{self, BufWriter, StdoutLock},
    str,
};

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

fn get_factorial(n: usize) -> usize {
    let mut ret = 1;

    for i in 1..=n {
        ret *= i;
    }

    ret
}

fn get_mobile(curr: &Vec<usize>, dir: &Vec<bool>, n: usize, m: usize) -> usize {
    let mut mobile_prev = 0;
    let mut mobile = 0;

    for i in 0..n * m {
        if !dir[curr[i] - 1] && i != 0 {
            if curr[i] > curr[i - 1] && curr[i] > mobile_prev {
                mobile = curr[i];
                mobile_prev = mobile;
            }
        }

        if dir[curr[i] - 1] && i != n * m - 1 {
            if curr[i] > curr[i + 1] && curr[i] > mobile_prev {
                mobile = curr[i];
                mobile_prev = mobile;
            }
        }
    }

    if mobile == 0 && mobile_prev == 0 {
        0
    } else {
        mobile
    }
}

fn search_arr(curr: &Vec<usize>, mobile: usize, n: usize, m: usize) -> usize {
    for i in 0..n * m {
        if curr[i] == mobile {
            return i + 1;
        }
    }

    0
}

fn print_one_term(
    out: &mut BufWriter<StdoutLock>,
    perm_map: &mut Vec<char>,
    curr: &mut Vec<usize>,
    dir: &mut Vec<bool>,
    n: usize,
    m: usize,
) {
    let mobile = get_mobile(curr, dir, n, m);
    let pos = search_arr(&curr, mobile, n, m);

    if dir[curr[pos - 1] - 1] {
        curr.swap(pos - 1, pos);
    } else {
        curr.swap(pos - 1, pos - 2);
    }

    for i in 0..n * m {
        if curr[i] > mobile {
            if dir[curr[i] - 1] {
                dir[curr[i] - 1] = false;
            } else {
                dir[curr[i] - 1] = true;
            }
        }
    }

    print_permutation_internal(out, perm_map, curr, n, m);
}

fn print_permutation(
    out: &mut BufWriter<StdoutLock>,
    perm_map: &mut Vec<char>,
    n: usize,
    m: usize,
) {
    let mut curr = vec![0; n * m];
    let mut dir = vec![false; n * m];

    for i in 0..n * m {
        curr[i] = i + 1;
    }

    print_permutation_internal(out, perm_map, &curr, n, m);

    for i in 0..n * m {
        dir[i] = false;
    }

    for _ in 1..get_factorial(n * m) {
        print_one_term(out, perm_map, &mut curr, &mut dir, n, m);
    }
}

fn print_permutation_internal(
    out: &mut BufWriter<StdoutLock>,
    perm_map: &mut Vec<char>,
    curr: &Vec<usize>,
    n: usize,
    m: usize,
) {
    let mut ret = vec![vec![' '; m]; n];

    for i in 0..n {
        for j in 0..m {
            let idx = curr[i * m + j];
            let ch = perm_map[idx];

            if i % 2 == 1 {
                ret[i][m - j - 1] = ch;
            } else {
                ret[i][j] = ch;
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![vec![' '; m]; n];
    let mut perm_map = vec![' '; n * m + 1];

    for i in 0..n {
        for j in 0..m {
            board[i][j] = ('1' as u8 + (i * m + j) as u8) as char;
        }
    }

    board[n - 1][m - 1] = '#';

    let mut idx = 1;

    for i in 0..n {
        if i % 2 == 1 {
            for j in (0..m).rev() {
                perm_map[idx] = board[i][j];
                idx += 1;
            }
        } else {
            for j in 0..m {
                perm_map[idx] = board[i][j];
                idx += 1;
            }
        }
    }

    print_permutation(&mut out, &mut perm_map, n, m);
}
