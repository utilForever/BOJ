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

    let num_switches = scan.token::<usize>();
    let mut switches = vec![0; num_switches + 1];

    for i in 1..=num_switches {
        switches[i] = scan.token::<i64>();
    }

    let num_students = scan.token::<i64>();

    for _ in 0..num_students {
        let (gender, num) = (scan.token::<i64>(), scan.token::<usize>());

        if gender == 1 {
            for i in (num..=num_switches).step_by(num) {
                switches[i] = 1 - switches[i];
            }
        } else {
            let mut idx = 0;

            loop {
                if (num as i64 - idx as i64) <= 0 || num + idx > num_switches {
                    break;
                }

                if switches[num - idx] == switches[num + idx] {
                    idx += 1;
                } else {
                    break;
                }
            }

            idx -= 1;

            for i in num - idx..=num + idx {
                switches[i] = 1 - switches[i];
            }
        }
    }

    for i in 1..=num_switches {
        write!(out, "{} ", switches[i]).unwrap();

        if i % 20 == 0 {
            writeln!(out).unwrap();
        }
    }

    writeln!(out).unwrap();
}
