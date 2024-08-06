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

fn calculate_reward_max(
    damages: &Vec<i64>,
    bosses: &Vec<(i64, i64)>,
    k: usize,
    idx_character: usize,
    idx_boss: usize,
    time_used: i64,
    reward: i64,
) -> i64 {
    if time_used > 900 {
        return 0;
    } else if idx_boss == k {
        return reward;
    }

    let calculate_time = |damage: i64, boss_hp: i64| -> i64 {
        let mut time_to_kill = boss_hp / damage;

        if boss_hp % damage != 0 {
            time_to_kill += 1;
        }

        time_to_kill
    };

    let (boss_hp, boss_reward) = bosses[idx_boss];
    let ret1 = calculate_reward_max(
        damages,
        bosses,
        k,
        idx_character,
        idx_boss + 1,
        time_used,
        reward,
    );
    let ret2 = calculate_reward_max(
        damages,
        bosses,
        k,
        idx_character,
        idx_boss + 1,
        time_used + calculate_time(damages[idx_character], boss_hp),
        reward + boss_reward,
    );

    ret1.max(ret2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut damages = vec![0; n];
    let mut bosses = vec![(0, 0); k];

    for i in 0..n {
        damages[i] = scan.token::<i64>();
    }

    for i in 0..k {
        bosses[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = vec![0; n];

    for i in 0..n {
        ret[i] = calculate_reward_max(&damages, &bosses, k, i, 0, 0, 0);
    }

    ret.sort_by(|a, b| b.cmp(a));

    writeln!(out, "{}", ret[..m].iter().sum::<i64>()).unwrap();
}
