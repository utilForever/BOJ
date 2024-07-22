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

    let (n, g) = (scan.token::<usize>(), scan.token::<i64>());
    let mut rooms = vec![0; n];

    for _ in 0..g {
        let mut guests = scan.token::<i64>();

        while guests > 0 {
            let has_empty_room = rooms.iter().any(|&room| room == 0);

            if has_empty_room {
                for i in 0..n {
                    if rooms[i] == 0 {
                        if guests >= 2 {
                            rooms[i] = 2;
                            guests -= 2;
                        } else if guests == 1 {
                            rooms[i] = 1;
                            guests -= 1;
                        }
                    }
                }
            } else {
                for i in 0..n {
                    if rooms[i] == 1 && guests >= 1 {
                        rooms[i] = 2;
                        guests -= 1;
                    }
                }
            }
        }
    }

    for room in rooms {
        writeln!(out, "{room}").unwrap();
    }
}
