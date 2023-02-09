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

    let (_, x, n) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut notes = vec![vec![0; 9]; n];

    for i in 0..n {
        for j in 0..9 {
            notes[i][j] = scan.token::<i64>();
        }
    }

    let calculate_score = |notes: &Vec<i64>| -> i64 {
        let mut idx = 0;
        let mut ret = 0;
        let mut long_started = false;
        let mut long_ended = false;
        let mut len_note = 0;

        while idx < notes.len() {
            match notes[idx] {
                0 => idx += 1,
                1 => {
                    idx += 1;
                    len_note += 1;
                }
                2 => {
                    let mut should_calculate = false;

                    if idx == notes.len() - 1 {
                        should_calculate = true;

                        if notes.iter().all(|&x| x != 0) {
                            long_ended = true;
                        } else if long_started {
                            long_ended = true;
                        } else {
                            long_started = true;
                        }
                    } else if notes[idx + 1] == 0 {
                        should_calculate = true;
                        long_ended = true;
                    } else if notes[idx + 1] == 1 {
                        if long_started {
                            should_calculate = true;
                            long_ended = true;
                        } else {
                            long_started = true;
                            ret += len_note * 100;
                            len_note = 0;
                        }
                    } else if notes[idx + 1] == 2 {
                        should_calculate = true;
                        long_ended = true;

                        if !long_started {
                            idx += 1;                           
                            long_started = true;
                            ret += len_note * 100;
                            len_note = 0;    
                        }
                    }

                    idx += 1;

                    if !should_calculate {
                        continue;
                    }

                    if long_started && long_ended {
                        ret += 80 + x * (len_note + 2) / 6;
                        long_started = false;
                        long_ended = false;
                        len_note = 0;
                    } else if long_ended {
                        ret += x * (len_note + 1) / 6;
                        long_ended = false;
                        len_note = 0;
                    }
                }
                _ => unreachable!(),
            }
        }

        if long_started && !long_ended {
            ret += 80 + x * (len_note + 1) / 6;
        } else if !long_started && long_ended {
            ret += x * (len_note + 1) / 6;
        } else {
            ret += len_note * 100;
        }

        ret
    };

    let ret = (0..9)
        .map(|i| {
            let mut notes = notes.iter().map(|note| note[i]).collect::<Vec<i64>>();
            notes.reverse();
            calculate_score(&notes)
        })
        .sum::<i64>();

    writeln!(out, "{ret}").unwrap();
}
