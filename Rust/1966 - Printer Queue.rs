use io::Write;
use std::{collections::VecDeque, io, str};

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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut printer = VecDeque::new();

        for i in 0..n {
            let priority = scan.token::<usize>();
            printer.push_back((i, priority));
        }

        let mut cnt = 0;

        while !printer.is_empty() {
            let doc_to_print = printer.pop_front().unwrap();
            let mut can_print = true;

            for doc in printer.iter() {
                if doc_to_print.1 < doc.1 {
                    can_print = false;
                    break;
                }
            }

            if can_print {
                cnt += 1;

                if doc_to_print.0 == m {
                    break;
                }
            } else {
                printer.push_back(doc_to_print);
            }
        }

        writeln!(out, "{}", cnt).unwrap();
    }
}
