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

    let calculate_score = |dist: f64| -> i64 {
        if dist <= 3.0 {
            100
        } else if dist <= 6.0 {
            80
        } else if dist <= 9.0 {
            60
        } else if dist <= 12.0 {
            40
        } else if dist <= 15.0 {
            20
        } else {
            0
        }
    };

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut player1 = [(0.0, 0.0); 3];
        let mut player2 = [(0.0, 0.0); 3];

        for i in 0..3 {
            player1[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        for i in 0..3 {
            player2[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut score_player1 = 0;
        let mut score_player2 = 0;

        for (x, y) in player1 {
            let dist = (x.powi(2) + y.powi(2)).sqrt();
            score_player1 += calculate_score(dist);
        }

        for (x, y) in player2 {
            let dist = (x.powi(2) + y.powi(2)).sqrt();
            score_player2 += calculate_score(dist);
        }

        writeln!(
            out,
            "SCORE: {score_player1} to {score_player2}, {}.",
            if score_player1 > score_player2 {
                "PLAYER 1 WINS"
            } else if score_player1 < score_player2 {
                "PLAYER 2 WINS"
            } else {
                "TIE"
            }
        )
        .unwrap();
    }
}
