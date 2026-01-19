use io::Write;
use std::{collections::VecDeque, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn pack(hp: i64, s: i64, has_yawn: bool) -> usize {
    (((hp - 1) as usize) * 3 + s as usize) * 2 + if has_yawn { 1 } else { 0 }
}

fn unpack(idx: usize) -> (i64, i64, bool) {
    let has_yawn = idx % 2 == 1;
    let s = ((idx / 2) % 3) as i64;
    let hp = (idx / 6) as i64 + 1;

    (hp, s, has_yawn)
}

fn calculate_score(hp: i64, s: i64, r: i64, hp_max: i64, den: i64) -> i64 {
    let factor = match s {
        0 => 2,
        1 => 3,
        2 => 5,
        _ => unreachable!(),
    };

    let num = factor * (3 * hp_max - 2 * hp) * r;
    num.min(den)
}

const ACTIONS: [char; 4] = ['F', 'G', 'Y', 'S'];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (r, hp_max) = (scan.token::<i64>(), scan.token::<usize>());
        let f = scan.token::<i64>();
        let cnt_states = hp_max * 6;

        let mut dist = vec![-1; cnt_states];
        let mut prev = vec![usize::MAX; cnt_states];
        let mut prev_action = vec![' '; cnt_states];

        let start = pack(hp_max as i64, 0, false);
        let mut queue = VecDeque::new();

        dist[start] = 0;
        queue.push_back(start);

        while let Some(idx) = queue.pop_front() {
            let (hp, s, has_yawn) = unpack(idx);

            for &action in ACTIONS.iter() {
                let (mut hp_next, mut s_next, mut has_yawn_next) = (hp, s, false);

                match action {
                    'F' => {
                        hp_next = if hp <= f { 1 } else { hp - f };
                    }
                    'G' => {
                        if s == 0 {
                            s_next = 1;
                        }
                    }
                    'Y' => {
                        has_yawn_next = true;
                    }
                    'S' => {
                        // Do nothing
                    }
                    _ => unreachable!(),
                }

                if has_yawn && s_next == 0 {
                    s_next = 2;
                }

                let idx_next = pack(hp_next, s_next, has_yawn_next);

                if dist[idx_next] == -1 {
                    dist[idx_next] = dist[idx] + 1;
                    prev[idx_next] = idx;
                    prev_action[idx_next] = action;
                    queue.push_back(idx_next);
                }
            }
        }

        let den = 3 * hp_max as i64 * 255 * 2;
        let mut best_score = -1;
        let mut best_dist = i64::MAX;
        let mut best_state = start;

        for i in 0..cnt_states {
            if dist[i] < 0 {
                continue;
            }

            let (hp, s, _) = unpack(i);
            let score = calculate_score(hp, s, r, hp_max as i64, den);

            if score > best_score || (score == best_score && dist[i] < best_dist) {
                best_score = score;
                best_dist = dist[i];
                best_state = i;
            }
        }

        let mut commands = Vec::new();
        let mut state_curr = best_state;

        while state_curr != start {
            commands.push(prev_action[state_curr]);
            state_curr = prev[state_curr];
        }

        commands.reverse();
        commands.push('C');

        writeln!(out, "{}", commands.iter().collect::<String>()).unwrap();
    }
}
