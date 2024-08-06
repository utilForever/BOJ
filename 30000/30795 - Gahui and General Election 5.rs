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

#[derive(Default, Clone)]
struct CharacterInfo {
    character_name: String,
    group_name: String,
    team_name: String,
    rank: i64,
    tier: i64,
}

impl CharacterInfo {
    fn new(
        character_name: String,
        group_name: String,
        team_name: String,
        rank: i64,
        tier: i64,
    ) -> Self {
        Self {
            character_name,
            group_name,
            team_name,
            rank,
            tier,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut expected = vec![CharacterInfo::default(); 80];
    let mut actual = vec![CharacterInfo::default(); 80];

    for i in 0..80 {
        let mut character_name = scan.token::<String>();

        loop {
            let s = scan.token::<String>();

            if s == "Group" {
                break;
            }

            character_name.push_str(" ");
            character_name.push_str(&s);
        }

        let (group_name, _, team_name) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        expected[i] = CharacterInfo::new(
            character_name,
            group_name,
            team_name,
            i as i64 + 1,
            i as i64 / 16 + 1,
        );
    }

    for i in 0..80 {
        let mut character_name = scan.token::<String>();

        loop {
            let s = scan.token::<String>();

            if s == "Group" {
                break;
            }

            character_name.push_str(" ");
            character_name.push_str(&s);
        }

        let (group_name, _, team_name) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        actual[i] = CharacterInfo::new(
            character_name,
            group_name,
            team_name,
            i as i64 + 1,
            i as i64 / 16 + 1,
        );
    }

    let mut score_max = 0;
    let mut ret = CharacterInfo::default();

    for i in 0..80 {
        let mut score = 0;

        // Calculate score 1
        let (rank_expected1, tier_expected1) = match expected.iter().position(|x| {
            x.character_name == actual[i].character_name
                && x.team_name == actual[i].team_name
                && x.group_name == actual[i].group_name
        }) {
            Some(x) => (expected[x].rank, expected[x].tier),
            None => (81, 6),
        };
        let (rank_actual1, tier_actual1) = (actual[i].rank, actual[i].tier);

        score += (10000 * (tier_expected1 - tier_actual1)).max(0);

        // Calculate score 2
        if actual[i].rank % 16 == 1 {
            score += if tier_expected1 > tier_actual1 {
                20000
            } else {
                10000
            };
        }

        // Calculate score 3
        if tier_expected1 != 1 && tier_actual1 == 1 {
            let mut cnt = 0;

            for j in 0..16 {
                let character = expected[j].clone();

                let (rank_expected2, _) = (character.rank, character.tier);
                let (rank_actual2, tier_actual2) = match actual.iter().position(|x| {
                    x.character_name == character.character_name
                        && x.team_name == character.team_name
                        && x.group_name == character.group_name
                }) {
                    Some(x) => (actual[x].rank, actual[x].tier),
                    None => (81, 6),
                };

                if tier_actual2 == 1 {
                    continue;
                }

                if rank_expected1 <= rank_expected2 {
                    continue;
                }

                if rank_actual1 >= rank_actual2 {
                    continue;
                }

                cnt += 1;
            }

            score += 30000 * cnt;
        }

        if score_max < score {
            score_max = score;
            ret = actual[i].clone();
        }
    }

    writeln!(out, "{}", ret.group_name).unwrap();
    writeln!(out, "{}", ret.team_name).unwrap();
    writeln!(out, "{}", ret.character_name).unwrap();
}
