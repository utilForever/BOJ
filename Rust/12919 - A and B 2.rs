use io::Write;
use std::{collections::HashSet, io, str};

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

fn process(set: &mut HashSet<String>, s: &String, t: Vec<char>, is_found: &mut bool) {
    if *is_found {
        return;
    }

    if s.len() == t.len() {
        if t.iter().collect::<String>() == *s {
            *is_found = true;
        }

        return;
    }

    // Case 1
    if *t.last().unwrap() == 'A' {
        let mut t_clone = t.clone();
        t_clone.pop();

        if !set.contains(&t_clone.iter().collect::<String>()) {
            set.insert(t_clone.iter().collect::<String>());
            process(set, s, t_clone, is_found);
        }
    }

    // Case 2
    if t[0] == 'B' {
        let mut t_clone = t.clone();
        t_clone.reverse();
        t_clone.pop();

        if !set.contains(&t_clone.iter().collect::<String>()) {
            set.insert(t_clone.iter().collect::<String>());
            process(set, s, t_clone, is_found);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (s, t) = (scan.token::<String>(), scan.token::<String>());
    let t = t.chars().collect::<Vec<_>>();

    let mut set = HashSet::new();
    let mut is_found = false;

    set.insert(t.iter().collect::<String>());

    process(&mut set, &s, t, &mut is_found);

    writeln!(out, "{}", if is_found { 1 } else { 0 }).unwrap();
}
