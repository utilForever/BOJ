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

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let mut map_team_member = HashMap::new();
    let mut map_member_team = HashMap::new();

    for _ in 0..n {
        let team = scan.token::<String>();
        let num = scan.token::<i64>();

        for _ in 0..num {
            let member = scan.token::<String>();

            map_team_member
                .entry(team.clone())
                .or_insert(Vec::new())
                .push(member.clone());
            map_member_team.insert(member.clone(), team.clone());
        }
    }

    for team in map_team_member.iter_mut() {
        team.1.sort();
    }

    for _ in 0..m {
        let name = scan.token::<String>();
        let quiz = scan.token::<i64>();

        if quiz == 0 {
            for member in map_team_member.get(&name).unwrap() {
                writeln!(out, "{member}").unwrap();
            }
        } else {
            writeln!(out, "{}", map_member_team.get(&name).unwrap()).unwrap();
        }
    }
}
