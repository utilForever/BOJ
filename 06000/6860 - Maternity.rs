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

    let mother = scan.token::<String>().trim().chars().collect::<Vec<_>>();
    let father = scan.token::<String>().trim().chars().collect::<Vec<_>>();
    let mut child = vec![Vec::new(); 5];

    for i in 0..5 {
        let mother_local = [mother[i * 2], mother[i * 2 + 1]];
        let father_local = [father[i * 2], father[i * 2 + 1]];

        for j in 0..2 {
            for k in 0..2 {
                if mother_local[j] == (b'a' + i as u8) as char && mother_local[j] == father_local[k]
                {
                    child[i].push((b'a' + i as u8) as char);
                } else {
                    child[i].push((b'A' + i as u8) as char);
                }
            }
        }
    }

    let x = scan.token::<i64>();

    for _ in 0..x {
        let baby = scan.token::<String>().trim().chars().collect::<Vec<_>>();
        let mut check = true;

        for (i, c) in baby.iter().enumerate() {
            if !child[i].contains(c) {
                check = false;
                break;
            }
        }

        writeln!(
            out,
            "{}",
            if check {
                "Possible baby."
            } else {
                "Not their baby!"
            }
        )
        .unwrap();
    }
}
