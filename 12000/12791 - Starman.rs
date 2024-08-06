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

    let albums = [
        (1967, "DavidBowie"),
        (1969, "SpaceOddity"),
        (1970, "TheManWhoSoldTheWorld"),
        (1971, "HunkyDory"),
        (1972, "TheRiseAndFallOfZiggyStardustAndTheSpidersFromMars"),
        (1973, "AladdinSane"),
        (1973, "PinUps"),
        (1974, "DiamondDogs"),
        (1975, "YoungAmericans"),
        (1976, "StationToStation"),
        (1977, "Low"),
        (1977, "Heroes"),
        (1979, "Lodger"),
        (1980, "ScaryMonstersAndSuperCreeps"),
        (1983, "LetsDance"),
        (1984, "Tonight"),
        (1987, "NeverLetMeDown"),
        (1993, "BlackTieWhiteNoise"),
        (1995, "1.Outside"),
        (1997, "Earthling"),
        (1999, "Hours"),
        (2002, "Heathen"),
        (2003, "Reality"),
        (2013, "TheNextDay"),
        (2016, "BlackStar"),
    ];

    let q = scan.token::<_>();

    for _ in 0..q {
        let (s, e) = (scan.token::<i64>(), scan.token::<i64>());
        let mut ret = Vec::new();

        for album in albums.iter() {
            if album.0 >= s && album.0 <= e {
                ret.push(album);
            }
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for album in ret {
            writeln!(out, "{} {}", album.0, album.1).unwrap();
        }
    }
}
