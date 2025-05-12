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

const POS_MAX: usize = 100;
const CELLS: usize = (POS_MAX + 1) * (POS_MAX + 1);

fn get_cell(x: usize, y: usize) -> usize {
    x * (POS_MAX + 1) + y
}

fn is_singleton(
    set: &Vec<u64>,
    is_sum_odd: &Vec<bool>,
    base: usize,
    words: usize,
) -> Option<usize> {
    let mut ret = None;

    for i in 0..words {
        let word = set[base + i];

        if word == 0 {
            continue;
        }

        if word & (word - 1) != 0 {
            return None;
        }

        if ret.is_some() {
            return None;
        }

        ret = Some(i * 64 + word.trailing_zeros() as usize);
    }

    ret.filter(|&i| is_sum_odd[i])
}

// Reference: "Petrozavodsk Summer 2019. Day 6. MIPT Contest" Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let words = (n + 63) / 64;

        let mut meet = vec![0; CELLS * words];
        let mut set_right = vec![0; CELLS * words];
        let mut set_up = vec![0; CELLS * words];
        let mut is_sum_odd = vec![false; n];

        for i in 0..n {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
            let val = (x + y + 1) / 2;

            is_sum_odd[i] = (x + y) % 2 == 1;

            if val > POS_MAX {
                continue;
            }

            for xx in 0..=val {
                let yy = val - xx;

                if x >= xx && y >= yy {
                    let cell = get_cell(xx, yy) * words + i / 64;
                    meet[cell] |= 1u64 << (i & 63);
                }
            }
        }

        for x in (0..=POS_MAX).rev() {
            for y in (0..=POS_MAX).rev() {
                let cell = get_cell(x, y);
                let dst = cell * words;

                let (right_empty, right_single) = if x < POS_MAX {
                    let src = get_cell(x + 1, y) * words;
                    let mut empty = true;

                    for i in 0..words {
                        set_right[dst + i] = set_right[src + i] | meet[src + i];

                        if set_right[dst + i] != 0 {
                            empty = false;
                        }
                    }

                    (empty, is_singleton(&set_right, &is_sum_odd, dst, words))
                } else {
                    for i in 0..words {
                        set_right[dst + i] = 0;
                    }

                    (true, None)
                };

                let (up_empty, up_single) = if y < POS_MAX {
                    let src = get_cell(x, y + 1) * words;
                    let mut empty = true;

                    for i in 0..words {
                        set_up[dst + i] = set_up[src + i] | meet[src + i];

                        if set_up[dst + i] != 0 {
                            empty = false;
                        }
                    }

                    (empty, is_singleton(&set_up, &is_sum_odd, dst, words))
                } else {
                    for i in 0..words {
                        set_up[dst + i] = 0;
                    }

                    (true, None)
                };

                if right_empty || up_empty {
                    for i in 0..words {
                        set_right[dst + i] = 0;
                        set_up[dst + i] = 0;
                    }

                    continue;
                }

                if let Some(val) = right_single {
                    set_up[dst + val / 64] &= !(1u64 << (val & 63));
                }

                if let Some(val) = up_single {
                    set_right[dst + val / 64] &= !(1u64 << (val & 63));
                }
            }
        }

        let start = get_cell(0, 0) * words;
        let ret = set_right[start..start + words].iter().all(|&x| x == 0)
            && set_up[start..start + words].iter().all(|&x| x == 0);

        writeln!(out, "{}", if ret { "Yes" } else { "No" }).unwrap();
    }
}
