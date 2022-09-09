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

#[derive(Clone)]
struct Monster {
    pub row: usize,
    pub column: usize,
    pub attack: i64,
    pub defense: i64,
    pub cur_health: i64,
    pub max_health: i64,
    pub experience: i64,
}

impl Monster {
    pub fn new(
        row: usize,
        column: usize,
        attack: i64,
        defense: i64,
        cur_health: i64,
        max_health: i64,
        experience: i64,
    ) -> Self {
        Self {
            row,
            column,
            attack,
            defense,
            cur_health,
            max_health,
            experience,
        }
    }
}

#[derive(Clone)]
enum ItemType {
    Weapon(i64),
    Armor(i64),
    Accessory(String),
}

#[derive(Clone)]
struct Item {
    pub row: usize,
    pub column: usize,
    pub equip_type: ItemType,
}

impl Item {
    pub fn new(row: usize, column: usize, equip_type: ItemType) -> Self {
        Self {
            row,
            column,
            equip_type,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![' '; m]; n];
    let mut num_monsters = 0;
    let mut num_items = 0;

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            grid[i][j] = c;

            if c == '&' || c == 'M' {
                num_monsters += 1;
            } else if c == 'B' {
                num_items += 1;
            }
        }
    }

    let s = scan.token::<String>();
    let moves = s.chars().collect::<Vec<_>>();

    let mut monsters = Vec::new();

    for _ in 0..num_monsters {
        let (r, c, s, w, a, h, e) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        monsters.push(Monster::new(r, c, s, w, a, h, e));
    }

    let mut items = Vec::new();

    for _ in 0..num_items {
        let (r, c, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<String>(),
        );

        if t == "W" {
            let s = scan.token::<i64>();
            items.push(Item::new(r, c, ItemType::Weapon(s)));
        } else if t == "A" {
            let s = scan.token::<i64>();
            items.push(Item::new(r, c, ItemType::Armor(s)));
        } else {
            let s = scan.token::<String>();
            items.push(Item::new(r, c, ItemType::Accessory(s)));
        }
    }
}
