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

    for _ in 0..4 {
        let _ = scan.token::<String>();
    }

    let n = scan.token::<usize>();
    let mut candidates = vec![(String::new(), false); n];

    for i in 0..n {
        candidates[i].0 = scan.token::<String>();
    }

    candidates.push(("swi".to_string(), false));

    let m = scan.token::<usize>();
    let mut stories = vec![String::new(); m];

    for i in 0..m {
        stories[i] = scan.token::<String>();

        if let Some(pos) = candidates.iter().position(|(name, _)| *name == stories[i]) {
            candidates[pos].1 = true;
        }
    }

    candidates.sort();

    writeln!(
        out,
        "{}",
        if candidates
            .iter()
            .find(|&(name, _)| name == "dongho")
            .is_some()
        {
            "dongho"
        } else if candidates
            .iter()
            .filter(|&(_, has_story)| *has_story == false)
            .count()
            == 1
        {
            candidates
                .iter()
                .find(|&(_, has_story)| *has_story == false)
                .unwrap()
                .0
                .as_str()
        } else if candidates
            .iter()
            .filter(|&(name, has_story)| name == "bumin" && *has_story == false)
            .count()
            == 1
        {
            "bumin"
        } else if candidates
            .iter()
            .filter(|&(name, has_story)| name == "cake" && *has_story == false)
            .count()
            == 1
        {
            "cake"
        } else if candidates
            .iter()
            .filter(|&(name, has_story)| name == "lawyer" && *has_story == false)
            .count()
            == 1
        {
            "lawyer"
        } else {
            candidates
                .iter()
                .filter(|&(name, has_story)| name != "swi" && *has_story == false)
                .nth(0)
                .unwrap()
                .0
                .as_str()
        }
    )
    .unwrap();
}
