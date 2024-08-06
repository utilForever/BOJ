use io::Write;
use std::{io, str, vec};

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

fn backtrack(
    characters: &Vec<char>,
    visited: &mut Vec<bool>,
    ret: &mut Vec<String>,
    limit: i64,
    index: usize,
    count: i64,
) {
    if count == limit {
        let characters_selected = characters
            .iter()
            .zip(visited.iter())
            .filter(|(_, &v)| v)
            .map(|(&c, _)| c)
            .collect::<Vec<char>>();

        let cnt_vowels = characters_selected
            .iter()
            .filter(|&&c| "aeiou".contains(c))
            .count();
        let cnt_consonants = characters_selected.len() - cnt_vowels;

        if cnt_vowels >= 1 && cnt_consonants >= 2 {
            ret.push(characters_selected.iter().collect::<String>());
        }

        return;
    }

    for i in index..characters.len() {
        visited[i] = true;
        backtrack(characters, visited, ret, limit, i + 1, count + 1);
        visited[i] = false;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, c) = (scan.token::<i64>(), scan.token::<usize>());
    let mut characters = vec![' '; c];

    for i in 0..c {
        characters[i] = scan.token::<char>();
    }

    characters.sort();

    let mut visited = vec![false; c];
    let mut ret = Vec::new();
    backtrack(&characters, &mut visited, &mut ret, l, 0, 0);

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
