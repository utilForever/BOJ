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

    let n = scan.token::<usize>();
    let mut favorites = vec![(0, 0); n];
    let mut favorites_a = vec![(0, 0); n];
    let mut favorites_b = vec![(0, 0); n];
    let mut favorites_c = vec![(0, 0); n];
    let mut favorites_d = vec![(0, 0); n];

    for i in 0..n {
        let (a, b, c, d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        favorites[i] = (i, a.max(b).max(c));
        favorites_a[i] = (i, a);
        favorites_b[i] = (i, b);
        favorites_c[i] = (i, c);
        favorites_d[i] = (i, d);
    }

    favorites.sort_by(|a, b| b.1.cmp(&a.1));
    favorites_a.sort_by(|a, b| b.1.cmp(&a.1));
    favorites_b.sort_by(|a, b| b.1.cmp(&a.1));
    favorites_c.sort_by(|a, b| b.1.cmp(&a.1));
    favorites_d.sort_by(|a, b| b.1.cmp(&a.1));

    favorites_a.truncate(11);
    favorites_b.truncate(11);
    favorites_c.truncate(11);
    favorites_d.truncate(11);

    let mut ret = 0;

    for player_a in favorites_a.iter() {
        for player_b in favorites_b.iter() {
            if player_a.0 == player_b.0 {
                continue;
            }

            for player_c in favorites_c.iter() {
                if player_a.0 == player_c.0 || player_b.0 == player_c.0 {
                    continue;
                }

                for player_d in favorites_d.iter() {
                    if player_a.0 == player_d.0
                        || player_b.0 == player_d.0
                        || player_c.0 == player_d.0
                    {
                        continue;
                    }

                    let mut favorites_total = player_a.1 + player_b.1 + player_c.1 + player_d.1;
                    let mut player_cnt = 4;
                    let mut idx = 0;

                    while player_cnt < 11 {
                        if favorites[idx].0 == player_a.0
                            || favorites[idx].0 == player_b.0
                            || favorites[idx].0 == player_c.0
                            || favorites[idx].0 == player_d.0
                        {
                            idx += 1;
                            continue;
                        }

                        favorites_total += favorites[idx].1;
                        player_cnt += 1;
                        idx += 1;
                    }

                    ret = ret.max(favorites_total);
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
