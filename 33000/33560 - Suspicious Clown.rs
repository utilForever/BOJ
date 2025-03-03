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

    let n = scan.token::<usize>();
    let mut dices = vec![0; n];

    for i in 0..n {
        dices[i] = scan.token::<i64>();
    }

    let mut idx = 0;
    let mut ret = [0; 4];

    while idx < n {
        let mut score = 0;
        let mut point = 1;
        let mut time = 0;
        let mut time_step = 4;
        let mut finished = false;

        while idx < n {
            if time > 240 {
                finished = true;
                break;
            }

            let dice = dices[idx];
            idx += 1;

            match dice {
                1 => {
                    finished = true;
                    break;
                }
                2 => {
                    if point > 1 {
                        point /= 2;
                    } else {
                        time_step += 2;
                    }
                }
                3 => {
                    // Do nothing
                }
                4 => {
                    // Do nothing (time increase later)
                }
                5 => {
                    if time_step > 1 {
                        time_step -= 1;
                    }
                }
                6 => {
                    if point < 32 {
                        point *= 2;
                    }
                }
                _ => unreachable!(),
            }

            score += point;

            time += if dice == 4 { time_step + 56 } else { time_step };
        }

        if finished {
            if score >= 35 && score < 65 {
                ret[0] += 1;
            } else if score >= 65 && score < 95 {
                ret[1] += 1;
            } else if score >= 95 && score < 125 {
                ret[2] += 1;
            } else if score >= 125 {
                ret[3] += 1;
            }
        }
    }

    writeln!(out, "{}\n{}\n{}\n{}", ret[0], ret[1], ret[2], ret[3]).unwrap();
}
