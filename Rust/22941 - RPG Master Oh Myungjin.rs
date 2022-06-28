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

    let (player_hp, player_atk, mut boss_hp, boss_atk) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let (p, s) = (scan.token::<i64>(), scan.token::<i64>());

    if player_atk == 1 || (boss_hp % player_atk >= 1 && boss_hp % player_atk <= p) {
        boss_hp += s;
    }

    let hp_player = player_hp / boss_atk + if player_hp % boss_atk == 0 { 0 } else { 1 };
    let hp_boss = boss_hp / player_atk + if boss_hp % player_atk == 0 { 0 } else { 1 } - 1;

    writeln!(
        out,
        "{}",
        if hp_player > hp_boss {
            "Victory!"
        } else {
            "gg"
        }
    )
    .unwrap();
}
