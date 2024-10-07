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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let mut n = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut ret = String::new();

        for i in 0..n.len() - 1 {
            if n[i] > n[i + 1] {
                if n[i] == '1' {
                    ret.clear();

                    for _ in 0..i {
                        ret.push('9');    
                    }

                    for j in i + 1..n.len() {
                        n[j] = '9';
                    }
                } else {
                    n[i] = (n[i] as u8 - 1) as char;

                    let mut idx = i as i64 - 1;

                    while idx >= 0 {
                        if n[idx as usize] > n[idx as usize + 1] {
                            n[idx as usize] = (n[idx as usize] as u8 - 1) as char;
                            idx -= 1;
                        } else {
                            break;
                        }
                    }

                    if idx != i as i64 - 1 && idx >= 0 {
                        let len = i - idx as usize - 2;
                        let mut s = "9".repeat(len);
                        
                        s.insert(0, n[idx as usize + 1]);
                        s.insert(0, n[idx as usize]);
                        ret.replace_range(idx as usize.., s.as_str());
                    } else if idx != i as i64 - 1 && idx < 0 {
                        ret.clear();
                        ret.push(n[0]);

                        for _ in 1..i {
                            ret.push('9');    
                        }
                    }

                    for j in idx as usize + 2..n.len() {
                        n[j] = '9';
                    }    

                    ret.push(n[i]);
                }
            } else {
                ret.push(n[i]);
            }
        }

        ret.push(*n.last().unwrap());

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
