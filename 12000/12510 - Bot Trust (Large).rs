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
        let n = scan.token::<usize>();
        let mut commands = vec![(' ', 0); n];

        for j in 0..n {
            let (r, p) = (scan.token::<char>(), scan.token::<i64>());            
            commands[j] = (r, p);
        }

        let mut idx_command = 0;
        let mut pos_orange = 1;
        let mut pos_blue = 1;
        let mut time = 1;

        loop {
            let (r, p) = commands[idx_command];

            if r == 'O' {
                if pos_orange < p {
                    pos_orange += 1;
                } else if pos_orange > p {
                    pos_orange -= 1;
                } else {
                    idx_command += 1;
                }

                if let Some(command_blue) = commands[idx_command..].iter().position(|&x| x.0 == 'B') {
                    if pos_blue < commands[idx_command + command_blue].1 {
                        pos_blue += 1;
                    } else if pos_blue > commands[idx_command + command_blue].1 {
                        pos_blue -= 1;
                    }
                }
            } else {
                if pos_blue < p {
                    pos_blue += 1;
                } else if pos_blue > p {
                    pos_blue -= 1;
                } else {
                    idx_command += 1;
                }

                if let Some(command_orange) = commands[idx_command..].iter().position(|&x| x.0 == 'O') {
                    if pos_orange < commands[idx_command + command_orange].1 {
                        pos_orange += 1;
                    } else if pos_orange > commands[idx_command + command_orange].1 {
                        pos_orange -= 1;
                    }
                }
            }

            if idx_command == n {
                break;
            }

            time += 1;
        }

        writeln!(out, "Case #{i}: {time}").unwrap();
    }
}
