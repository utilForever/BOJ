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

    let (r, _) = (scan.token::<usize>(), scan.token::<usize>());
    let mut people = Vec::new();
    let mut seats = Vec::new();

    for i in 0..r {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            if c == 'L' {
                seats.push((i, j));
            } else if c == 'X' {
                people.push((i, j));
            }
        }
    }

    let mut dists = Vec::new();

    for seat in seats {
        let mut map_dist: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();

        for person in &people {
            let dist =
                (seat.0 as i64 - person.0 as i64).pow(2) + (seat.1 as i64 - person.1 as i64).pow(2);

            if map_dist.contains_key(&dist) {
                map_dist.get_mut(&dist).unwrap().push(*person);
            } else {
                map_dist.insert(dist, vec![*person]);
            }
        }

        let mut map_dist = map_dist
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Vec<_>>();
        map_dist.sort_by(|a, b| a.0.cmp(&b.0));
        dists.push((seat, map_dist));
    }

    let mut ret = 0;

    while !dists.is_empty() {
        let dist_min = dists
            .iter()
            .min_by(|a, b| a.1[0].0.cmp(&b.1[0].0))
            .unwrap()
            .clone();

        if dist_min.1.is_empty() {
            break;
        }

        let cnt_dist_min = dist_min.1[0].1.len();
        if cnt_dist_min > 1 {
            ret += 1;
        }

        dists.retain(|x| x.0 != dist_min.0);

        let coords_people_min = dist_min.1[0].1.clone();

        dists.iter_mut().for_each(|val| {
            val.1.iter_mut().for_each(|val| {
                val.1.retain(|x| !coords_people_min.contains(x));
            });

            val.1.retain(|x| !x.1.is_empty());
        });
    }

    writeln!(out, "{ret}").unwrap();
}
