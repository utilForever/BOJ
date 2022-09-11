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

#[derive(Clone)]
struct Player {
    pub row: usize,
    pub column: usize,
    pub level: i64,
    pub attack: i64,
    pub defense: i64,
    pub cur_health: i64,
    pub max_health: i64,
    pub experience: i64,
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessory: HashMap<String, Item>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            row: 0,
            column: 0,
            level: 1,
            attack: 2,
            defense: 2,
            cur_health: 20,
            max_health: 20,
            experience: 0,
            weapon: None,
            armor: None,
            accessory: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct Monster {
    pub row: usize,
    pub column: usize,
    pub name: String,
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
        name: String,
        attack: i64,
        defense: i64,
        max_health: i64,
        experience: i64,
    ) -> Self {
        Self {
            row,
            column,
            name,
            attack,
            defense,
            cur_health: max_health,
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

fn pick_item_box(player: &mut Player, items: &Vec<Item>) -> Result<(), String> {
    let item = items
        .iter()
        .find(|item| item.row == player.row && item.column == player.column)
        .ok_or("No item found")?;

    match &item.equip_type {
        ItemType::Weapon(attack) => {
            if let Some(weapon) = &player.weapon {
                if weapon.equip_type == ItemType::Weapon(*attack) {
                    return Err("You already have this weapon".to_string());
                }
            }

            player.weapon = Some(item.clone());
            player.attack += attack;
        }
        ItemType::Armor(defense) => {
            if let Some(armor) = &player.armor {
                if armor.equip_type == ItemType::Armor(*defense) {
                    return Err("You already have this armor".to_string());
                }
            }

            player.armor = Some(item.clone());
            player.defense += defense;
        }
        ItemType::Accessory(name) => {
            if player.accessory.contains_key(name) {
                return Err("You already have this accessory".to_string());
            }

            player.accessory.insert(name.clone(), item.clone());
        }
    }

    Ok(())
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![' '; m + 1]; n + 1];
    let mut player = Player::new();
    let mut num_monsters = 0;
    let mut num_items = 0;

    // Input grid
    for i in 1..=n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            grid[i][j + 1] = c;

            if c == '&' || c == 'M' {
                num_monsters += 1;
            } else if c == 'B' {
                num_items += 1;
            } else if c == '@' {
                player.row = i;
                player.column = j + 1;
            }
        }
    }

    // Input moves
    let s = scan.token::<String>();
    let moves = s.chars().collect::<Vec<_>>();

    let mut monsters = Vec::new();

    // Input monster information
    for _ in 0..num_monsters {
        let (r, c, s, w, a, h, e) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<String>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        monsters.push(Monster::new(r, c, s, w, a, h, e));
    }

    let mut items = Vec::new();

    // Input item information
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

    let mut passed_turns = 0;

    // Process move
    for &move_type in moves.iter() {
        passed_turns += 1;

        // Calculate next position
        let (mut next_row, mut next_column) = (player.row, player.column);

        if move_type == 'U' {
            next_row -= 1;
        } else if move_type == 'D' {
            next_row += 1;
        } else if move_type == 'L' {
            next_column -= 1;
        } else if move_type == 'R' {
            next_column += 1;
        }

        // Check if player can move
        if next_row >= 1
            && next_row <= n
            && next_column >= 1
            && next_column <= m
            && grid[next_row][next_column] != '#'
        {
            player.row = next_row;
            player.column = next_column;
        }

        // Process logic according to the cell
        let ret = match grid[player.row][player.column] {
            '.' => Ok(()),
            'B' => pick_item_box(&mut player, &mut items),
            '^' => take_damage_by_spike(&mut player),
            '&' => battle_with_monster(&mut player, &mut monsters),
            'M' => battle_with_boss(&mut player, &mut monsters),
            _ => Ok(()),
        };
    }

    // Print output
    for i in 1..=n {
        for j in 1..=m {
            write!(out, "{}", grid[i][j]).unwrap();
        }
        writeln!(out).unwrap();
    }

    writeln!(out, "Passed Turns : {passed_turns}").unwrap();
    writeln!(out, "LV : {}", player.level).unwrap();
    writeln!(out, "HP : {}/{}", player.cur_health, player.max_health).unwrap();
    writeln!(
        out,
        "ATT : {}+{}",
        player.attack,
        player
            .weapon
            .as_ref()
            .map(|w| match w.equip_type {
                ItemType::Weapon(s) => s,
                _ => 0,
            })
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        out,
        "DEF : {}+{}",
        player.defense,
        player
            .armor
            .as_ref()
            .map(|a| match a.equip_type {
                ItemType::Armor(s) => s,
                _ => 0,
            })
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(out, "EXP : {}/{}", player.experience, player.level * 5).unwrap();
}
