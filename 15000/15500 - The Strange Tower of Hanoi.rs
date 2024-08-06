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

enum Turn {
    Stack1,
    Stack2,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();

    for _ in 0..n {
        let num = scan.token::<i64>();
        stack1.push(num);
    }

    let mut idx_disk = n;
    let mut turn = Turn::Stack1;
    let mut ret = Vec::new();

    while idx_disk > 0 {
        match turn {
            Turn::Stack1 => {
                while !stack1.is_empty() {
                    if *stack1.last().unwrap() == idx_disk {
                        ret.push((1, 3));
                        stack1.pop();
                        idx_disk -= 1;
                    } else {
                        ret.push((1, 2));
                        stack2.push(*stack1.last().unwrap());
                        stack1.pop();
                    }
                }

                turn = Turn::Stack2;
            }
            Turn::Stack2 => {
                while !stack2.is_empty() {
                    if *stack2.last().unwrap() == idx_disk {
                        ret.push((2, 3));
                        stack2.pop();
                        idx_disk -= 1;
                    } else {
                        ret.push((2, 1));
                        stack1.push(*stack2.last().unwrap());
                        stack2.pop();
                    }
                }

                turn = Turn::Stack1;
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret.iter() {
        writeln!(out, "{} {}", val.0, val.1).unwrap();
    }
}
