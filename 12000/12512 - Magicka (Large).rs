use io::Write;
use std::{collections::HashMap, io, str};

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
        let c = scan.token::<usize>();
        let mut combinations = HashMap::new();

        for _ in 0..c {
            let combination = scan.token::<String>().chars().collect::<Vec<_>>();
            let (a, b, ret) = (combination[0], combination[1], combination[2]);

            combinations.insert((a, b), ret);
            combinations.insert((b, a), ret);
        }

        let d = scan.token::<usize>();
        let mut oppositions = HashMap::new();

        for _ in 0..d {
            let opposition = scan.token::<String>().chars().collect::<Vec<_>>();
            let (a, b) = (opposition[0], opposition[1]);

            oppositions.entry(a).or_insert(Vec::new()).push(b);
            oppositions.entry(b).or_insert(Vec::new()).push(a);
        }

        let _ = scan.token::<usize>();
        let invokes = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut ret = Vec::new();

        for element in invokes {
            ret.push(element);

            if ret.len() >= 2 {
                let last_first = ret[ret.len() - 1];
                let last_second = ret[ret.len() - 2];

                if let Some(elem_new) = combinations.get(&(last_second, last_first)) {
                    ret.pop();
                    ret.pop();
                    ret.push(*elem_new);

                    continue;
                }
            }

            if let Some(opposition) = oppositions.get(&element) {
                if ret.iter().any(|&x| opposition.contains(&x)) {
                    ret.clear();
                }
            }
        }

        write!(out, "Case #{i}: [").unwrap();

        for (idx, &element) in ret.iter().enumerate() {
            write!(out, "{}", element).unwrap();

            if idx < ret.len() - 1 {
                write!(out, ", ").unwrap();
            }
        }

        writeln!(out, "]").unwrap();
    }
}
