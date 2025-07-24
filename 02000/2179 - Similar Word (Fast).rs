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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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
    let mut words = vec![String::new(); n];

    for i in 0..n {
        words[i] = scan.token::<String>();
    }

    let mut prefixes: HashMap<String, Vec<usize>> = HashMap::new();

    for i in 0..n {
        for j in 1..=words[i].len() {
            let prefix = words[i][0..j].to_string();
            prefixes.entry(prefix).or_insert_with(Vec::new).push(i);
        }
    }

    let mut ret = (0, usize::MAX, usize::MAX);

    for (prefix, indices) in prefixes {
        if indices.len() == 1 {
            continue;
        }

        let mut indices = indices.clone();
        indices.sort_unstable();

        if ret.0 < prefix.len()
            || (ret.0 == prefix.len() && indices[0] < ret.1)
            || (ret.0 == prefix.len() && indices[0] == ret.1 && indices[1] < ret.2)
        {
            ret = (prefix.len(), indices[0], indices[1]);
        }
    }

    writeln!(out, "{}", words[ret.1]).unwrap();
    writeln!(out, "{}", words[ret.2]).unwrap();
}
