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

#[derive(Debug, Clone)]
enum Cell {
    Land(i64),
    GoldenKey,
    Start,
    Island,
    Charity,
    Spaceship,
}

#[derive(Debug, Clone)]
enum GoldenKey {
    TakeMoney(i64),
    GiveMoney(i64),
    Donate(i64),
    Move(usize),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Result {
    Win,
    Lose,
    Unknown,
}

fn process_move(
    board: &mut Vec<Cell>,
    is_owner: &mut Vec<bool>,
    golden_keys: &Vec<GoldenKey>,
    pos_player: &mut usize,
    money_player: &mut i64,
    money_charity: &mut i64,
    idx_golden_key: &mut usize,
    n: usize,
    w: i64,
    g: usize,
) -> bool {
    match board[*pos_player] {
        Cell::Start => {}
        Cell::Land(value) => {
            if *money_player >= value {
                *money_player -= value;
                is_owner[*pos_player] = true;
            }
        }
        Cell::GoldenKey => match golden_keys[*idx_golden_key] {
            GoldenKey::TakeMoney(value) => {
                *idx_golden_key = (*idx_golden_key + 1) % g;
                *money_player += value;
            }
            GoldenKey::GiveMoney(value) => {
                *idx_golden_key = (*idx_golden_key + 1) % g;

                if *money_player < value {
                    return false;
                }

                *money_player -= value;
            }
            GoldenKey::Donate(value) => {
                *idx_golden_key = (*idx_golden_key + 1) % g;

                if *money_player < value {
                    return false;
                }

                *money_player -= value;
                *money_charity += value;
            }
            GoldenKey::Move(value) => {
                let cnt_pass_start = (*pos_player + value) / (4 * (n - 1));
                let pos_next = (*pos_player + value) % (4 * (n - 1));

                *idx_golden_key = (*idx_golden_key + 1) % g;
                *money_player += w * cnt_pass_start as i64;
                *pos_player = pos_next;

                return process_move(
                    board,
                    is_owner,
                    golden_keys,
                    pos_player,
                    money_player,
                    money_charity,
                    idx_golden_key,
                    n,
                    w,
                    g,
                );
            }
        },
        Cell::Island => {}
        Cell::Charity => {
            *money_player += *money_charity;
            *money_charity = 0;
        }
        Cell::Spaceship => {}
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, s, w, g) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut board = vec![Cell::Land(0); 4 * (n - 1)];
    let mut is_owner = vec![false; 4 * (n - 1)];
    let mut golden_keys = vec![GoldenKey::TakeMoney(0); g];

    for i in 0..g {
        let (num, value) = (scan.token::<i64>(), scan.token::<i64>());

        match num {
            1 => golden_keys[i] = GoldenKey::TakeMoney(value),
            2 => golden_keys[i] = GoldenKey::GiveMoney(value),
            3 => golden_keys[i] = GoldenKey::Donate(value),
            4 => golden_keys[i] = GoldenKey::Move(value as usize),
            _ => unreachable!(),
        }
    }

    // Preprocess board
    board[0] = Cell::Start;
    board[n - 1] = Cell::Island;
    board[2 * (n - 1)] = Cell::Charity;
    board[3 * (n - 1)] = Cell::Spaceship;

    is_owner[0] = true;
    is_owner[n - 1] = true;
    is_owner[2 * (n - 1)] = true;
    is_owner[3 * (n - 1)] = true;

    for i in 0..4 * (n - 1) {
        if i == 0 || i == n - 1 || i == 2 * (n - 1) || i == 3 * (n - 1) {
            continue;
        }

        let type_land = scan.token::<char>();

        if type_land == 'L' {
            let value = scan.token::<i64>();
            board[i] = Cell::Land(value);
        } else {
            board[i] = Cell::GoldenKey;
            is_owner[i] = true;
        }
    }

    let num_throw_dices = scan.token::<usize>();
    let mut throw_dices = vec![(0, 0); num_throw_dices];

    for i in 0..num_throw_dices {
        throw_dices[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut money_player = s;
    let mut money_charity = 0;
    let mut pos_player = 0;
    let mut idx_golden_key = 0;
    let mut num_turns_in_island = 0;
    let mut result = Result::Unknown;

    for throw_dice in throw_dices {
        // Process Spaceship
        if pos_player == 3 * (n - 1) {
            pos_player = 0;
            money_player += w;
        }

        let is_double = throw_dice.0 == throw_dice.1;
        let pos_next = (pos_player + throw_dice.0 + throw_dice.1) % (4 * (n - 1));

        // Process Island
        if matches!(board[pos_player], Cell::Island) {
            if num_turns_in_island == 3 {
                num_turns_in_island = 0;
            } else if is_double {
                num_turns_in_island = 3;
                continue;
            } else {
                num_turns_in_island += 1;
                continue;
            }
        }

        // Check the player passes the start point
        let cnt_pass_start = (pos_player + throw_dice.0 + throw_dice.1) / (4 * (n - 1));
        money_player += w * cnt_pass_start as i64;

        pos_player = pos_next;

        let result_move = process_move(
            &mut board,
            &mut is_owner,
            &golden_keys,
            &mut pos_player,
            &mut money_player,
            &mut money_charity,
            &mut idx_golden_key,
            n,
            w,
            g,
        );

        if !result_move {
            result = Result::Lose;
            break;
        }
    }

    if result == Result::Unknown {
        if is_owner.iter().all(|&x| x) {
            result = Result::Win;
        } else {
            result = Result::Lose;
        }
    }

    writeln!(
        out,
        "{}",
        match result {
            Result::Win => "WIN",
            Result::Lose => "LOSE",
            Result::Unknown => unreachable!(),
        }
    )
    .unwrap();
}
