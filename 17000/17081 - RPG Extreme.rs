use io::Write;
use std::{cmp, collections::HashMap, io, str};

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

#[derive(Clone, Debug)]
enum ItemType {
    Weapon(i64),
    Armor(i64),
    Accessory(String),
}

#[derive(Clone, Debug)]
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

fn pick_item_box(player: &mut Player, items: &Vec<Item>) {
    let item = items
        .iter()
        .find(|item| item.row == player.row && item.column == player.column)
        .unwrap();

    match &item.equip_type {
        ItemType::Weapon(_) => {
            player.weapon = Some(item.clone());
        }
        ItemType::Armor(_) => {
            player.armor = Some(item.clone());
        }
        ItemType::Accessory(name) => {
            if player.accessory.len() < 4 && !player.accessory.contains_key(name) {
                player.accessory.insert(name.clone(), item.clone());
            }
        }
    }
}

fn take_damage_by_spike(player: &mut Player) {
    let spike_damage = if player.accessory.contains_key("DX") {
        1
    } else {
        5
    };

    player.cur_health -= spike_damage;
}

fn battle_with_monster(player: &mut Player, monster: &mut Monster) {
    let attack = player.attack
        + player.weapon.as_ref().map_or(0, |item| {
            if let ItemType::Weapon(weapon) = &item.equip_type {
                *weapon
            } else {
                0
            }
        });
    let defense = player.defense
        + player.armor.as_ref().map_or(0, |item| {
            if let ItemType::Armor(armor) = &item.equip_type {
                *armor
            } else {
                0
            }
        });

    let mut is_first_turn = true;
    let mut is_player_turn = true;

    loop {
        if is_player_turn {
            let damage = if is_first_turn {
                if player.accessory.contains_key("CO") {
                    if player.accessory.contains_key("DX") {
                        cmp::max(1, attack * 3 - monster.defense)
                    } else {
                        cmp::max(1, attack * 2 - monster.defense)
                    }
                } else {
                    cmp::max(1, attack - monster.defense)
                }
            } else {
                cmp::max(1, attack - monster.defense)
            };

            monster.cur_health -= damage;
        } else {
            let damage = cmp::max(1, monster.attack - defense);

            player.cur_health -= damage;
        }

        if player.cur_health <= 0 {
            break;
        } else if monster.cur_health <= 0 {
            let experience = if player.accessory.contains_key("EX") {
                (monster.experience as f64 * 1.2) as i64
            } else {
                monster.experience
            };

            player.experience += experience;

            if player.experience >= player.level * 5 {
                player.level += 1;
                player.attack += 2;
                player.defense += 2;
                player.max_health += 5;
                player.cur_health = player.max_health;
                player.experience = 0;
            }

            if player.accessory.contains_key("HR") {
                player.cur_health = cmp::min(player.max_health, player.cur_health + 3);
            }

            break;
        }

        if is_first_turn {
            is_first_turn = false;
        }

        is_player_turn = !is_player_turn;
    }
}

fn battle_with_boss(player: &mut Player, boss: &mut Monster) {
    if player.accessory.contains_key("HU") {
        player.cur_health = player.max_health;
    }

    let attack = player.attack
        + player.weapon.as_ref().map_or(0, |item| {
            if let ItemType::Weapon(weapon) = &item.equip_type {
                *weapon
            } else {
                0
            }
        });
    let defense = player.defense
        + player.armor.as_ref().map_or(0, |item| {
            if let ItemType::Armor(armor) = &item.equip_type {
                *armor
            } else {
                0
            }
        });

    let mut is_first_player_turn = true;
    let mut is_first_boss_turn = true;
    let mut is_player_turn = true;

    loop {
        if is_player_turn {
            let damage = if is_first_player_turn {
                if player.accessory.contains_key("CO") {
                    if player.accessory.contains_key("DX") {
                        cmp::max(1, attack * 3 - boss.defense)
                    } else {
                        cmp::max(1, attack * 2 - boss.defense)
                    }
                } else {
                    cmp::max(1, attack - boss.defense)
                }
            } else {
                cmp::max(1, attack - boss.defense)
            };

            boss.cur_health -= damage;

            if is_first_player_turn {
                is_first_player_turn = false;
            }
        } else {
            let damage = if is_first_boss_turn && player.accessory.contains_key("HU") {
                0
            } else {
                cmp::max(1, boss.attack - defense)
            };

            player.cur_health -= damage;

            if is_first_boss_turn {
                is_first_boss_turn = false;
            }
        }

        if player.cur_health <= 0 {
            break;
        } else if boss.cur_health <= 0 {
            let experience = if player.accessory.contains_key("EX") {
                (boss.experience as f64 * 1.2) as i64
            } else {
                boss.experience
            };

            player.experience += experience;

            if player.experience >= player.level * 5 {
                player.level += 1;
                player.attack += 2;
                player.defense += 2;
                player.max_health += 5;
                player.cur_health = player.max_health;
                player.experience = 0;
            }

            if player.accessory.contains_key("HR") {
                player.cur_health = cmp::min(player.max_health, player.cur_health + 3);
            }

            break;
        }

        is_player_turn = !is_player_turn;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![' '; m + 1]; n + 1];
    let mut player = Player::new();
    let mut init_position_player = (0, 0);
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
                init_position_player = (i, j + 1);
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

    let mut prev_tile = '.';
    let mut passed_turns = 0;
    let mut result_msg = "Press any key to continue.".to_string();

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
            grid[player.row][player.column] = prev_tile;
            prev_tile = grid[next_row][next_column];
            grid[next_row][next_column] = '@';

            player.row = next_row;
            player.column = next_column;
        }

        // Process logic according to the cell
        match prev_tile {
            'B' => pick_item_box(&mut player, &mut items),
            '^' => take_damage_by_spike(&mut player),
            '&' => {
                let mut monster = monsters
                    .iter_mut()
                    .find(|monster| monster.row == player.row && monster.column == player.column)
                    .unwrap();
                battle_with_monster(&mut player, &mut monster)
            }
            'M' => {
                let mut boss = monsters
                    .iter_mut()
                    .find(|monster| monster.row == player.row && monster.column == player.column)
                    .unwrap();
                battle_with_boss(&mut player, &mut boss)
            }
            _ => (),
        };

        // Check if player is dead
        if player.cur_health <= 0 {
            if player.accessory.contains_key("RE") {
                player.accessory.remove("RE");

                let monster = monsters
                    .iter_mut()
                    .find(|monster| monster.row == player.row && monster.column == player.column);

                if let Some(monster) = monster {
                    monster.cur_health = monster.max_health;
                }

                grid[player.row][player.column] = prev_tile;
                prev_tile = grid[init_position_player.0][init_position_player.1];
                grid[init_position_player.0][init_position_player.1] = '@';

                player.row = init_position_player.0;
                player.column = init_position_player.1;
                player.cur_health = player.max_health;
            } else {
                grid[player.row][player.column] = prev_tile;
                player.cur_health = 0;
                result_msg = match grid[player.row][player.column] {
                    '^' => "YOU HAVE BEEN KILLED BY SPIKE TRAP..".to_string(),
                    '&' | 'M' => format!(
                        "YOU HAVE BEEN KILLED BY {}..",
                        monsters
                            .iter()
                            .find(|m| m.row == player.row && m.column == player.column)
                            .unwrap()
                            .name
                    ),
                    _ => String::new(),
                };

                break;
            }
        }

        prev_tile = match prev_tile {
            'B' => '.',
            '&' => {
                let monster = monsters
                    .iter_mut()
                    .find(|monster| monster.row == player.row && monster.column == player.column)
                    .unwrap();

                if monster.cur_health <= 0 {
                    '.'
                } else {
                    prev_tile
                }
            }
            _ => prev_tile,
        };

        if prev_tile == 'M' {
            let boss = monsters
                .iter_mut()
                .find(|monster| monster.row == player.row && monster.column == player.column)
                .unwrap();

            if boss.cur_health <= 0 {
                result_msg = "YOU WIN!".to_string();
                break;
            }
        }
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
    writeln!(out, "{result_msg}").unwrap();
}
