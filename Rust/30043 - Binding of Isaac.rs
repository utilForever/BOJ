use io::Write;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{self, BufWriter, StdoutLock},
    str,
};

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

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
enum RoomType {
    Empty,
    Start,
    Normal(i64),
    Boss,
    Secret,
    Treasure,
    Shop,
    Devil,
    Angel,
    Sacrifice,
    Curse,
}

struct BindingOfIsaac {
    seed: i64,
    board: Vec<Vec<RoomType>>,
}

impl BindingOfIsaac {
    fn new(seed: String) -> Self {
        let mut seed = seed.chars().collect::<Vec<_>>();
        seed.reverse();

        let radix = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect::<Vec<_>>();
        let m = 2_i64.pow(31);
        let mut ret = 0;

        while !seed.is_empty() {
            if *seed.last().unwrap() == '-' {
                seed.pop();
                continue;
            }

            ret *= 36;
            ret %= m;
            ret += radix
                .iter()
                .position(|&x| x == *seed.last().unwrap())
                .unwrap() as i64;
            seed.pop();
        }

        Self {
            seed: ret,
            board: vec![vec![RoomType::Empty; 9]; 9],
        }
    }

    fn rand(&mut self) -> i64 {
        let a = 1103515245;
        let c = 12345;
        let m = 2_i64.pow(31);
        let ret = self.seed;

        self.seed = (a * self.seed + c) % m;

        ret
    }

    fn rand_int(&mut self, l: i64, r: i64) -> i64 {
        l + self.rand() % (r - l + 1)
    }

    fn chance(&mut self, p: i64) -> bool {
        self.rand_int(1, 100) <= p
    }

    fn choice<T>(&mut self, arr: Vec<T>) -> T
    where
        T: Copy + Clone,
    {
        let idx = self.rand_int(0, arr.len() as i64 - 1);

        arr[idx as usize]
    }

    fn generate_dungeon(&mut self) {
        // Step 1
        let n = self.rand_int(10, 20);

        // Step 2
        let dy: [i64; 4] = [0, 1, 0, -1];
        let dx: [i64; 4] = [1, 0, -1, 0];
        let mut board = vec![vec![RoomType::Empty; 9]; 9];
        let mut room = Vec::new();

        board[4][4] = RoomType::Start;
        room.push((4, 4));

        let mut queue = VecDeque::new();
        let mut room_remain = n;

        while room_remain > 0 {
            queue.push_back(self.choice(room.clone()));

            while !queue.is_empty() {
                let (y, x) = queue.pop_front().unwrap();

                for i in 0..4 {
                    let y_next = y + dy[i];
                    let x_next = x + dx[i];

                    if room_remain == 0 {
                        continue;
                    }

                    if y_next < 0 || y_next >= 9 || x_next < 0 || x_next >= 9 {
                        continue;
                    }

                    let y_next = y_next as usize;
                    let x_next = x_next as usize;

                    if board[y_next][x_next] != RoomType::Empty {
                        continue;
                    }

                    let mut cnt = 0;

                    for j in 0..4 {
                        let y_next_next = y_next as i64 + dy[j];
                        let x_next_next = x_next as i64 + dx[j];

                        if y_next_next < 0
                            || y_next_next >= 9
                            || x_next_next < 0
                            || x_next_next >= 9
                        {
                            continue;
                        }

                        let y_next_next = y_next_next as usize;
                        let x_next_next = x_next_next as usize;

                        if board[y_next_next][x_next_next] == RoomType::Empty {
                            continue;
                        }

                        cnt += 1;
                    }

                    if cnt >= 2 {
                        continue;
                    }

                    if self.chance(50) == false {
                        continue;
                    }

                    room_remain -= 1;

                    board[y_next][x_next] = RoomType::Normal(0);
                    room.push((y_next as i64, x_next as i64));
                    queue.push_back((y_next as i64, x_next as i64));
                }
            }
        }

        room.remove(0);

        // Step 3
        let mut special = Vec::new();

        for &(y, x) in room.iter() {
            let mut cnt = 0;

            for i in 0..4 {
                let y_next = y as i64 + dy[i];
                let x_next = x as i64 + dx[i];

                if y_next < 0 || y_next >= 9 || x_next < 0 || x_next >= 9 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                if board[y_next][x_next] != RoomType::Empty {
                    cnt += 1;
                }
            }

            if cnt == 1 {
                special.push((y, x));
            }
        }

        // Assign Normal Room
        let mut require = Vec::new();

        for i in 0..10 {
            for _ in 0..10 - i {
                require.push(i);
            }
        }

        for &(y, x) in room.iter() {
            if special
                .iter()
                .filter(|&&(y_special, x_special)| y == y_special && x == x_special)
                .count()
                > 0
            {
                continue;
            }

            board[y as usize][x as usize] = RoomType::Normal(self.choice(require.clone()));
        }

        // Assign Boss Room
        let mut boss = Vec::new();

        for &(y, x) in special.iter() {
            let mut is_satisfied = true;

            for i in 0..4 {
                let y_next = y as i64 + dy[i];
                let x_next = x as i64 + dx[i];

                if y_next < 0 || y_next >= 9 || x_next < 0 || x_next >= 9 {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                if board[y_next][x_next] == RoomType::Start {
                    is_satisfied = false;
                    break;
                }
            }

            if is_satisfied {
                boss.push((y, x));
            }
        }

        let room_boss = self.choice(boss.clone());
        board[room_boss.0 as usize][room_boss.1 as usize] = RoomType::Boss;

        special.remove(special.iter().position(|&x| x == room_boss).unwrap());

        // Try to assign Secret Room
        if !special.is_empty() {
            let room_secret = self.choice(special.clone());
            board[room_secret.0 as usize][room_secret.1 as usize] = RoomType::Secret;

            special.remove(special.iter().position(|&x| x == room_secret).unwrap());
        }

        // Try to assign Treasure Room
        if !special.is_empty() {
            let room_treasure = self.choice(special.clone());
            board[room_treasure.0 as usize][room_treasure.1 as usize] = RoomType::Treasure;

            special.remove(special.iter().position(|&x| x == room_treasure).unwrap());

            if n >= 15 && !special.is_empty() && self.chance(25) {
                let room_treasure = self.choice(special.clone());
                board[room_treasure.0 as usize][room_treasure.1 as usize] = RoomType::Treasure;

                special.remove(special.iter().position(|&x| x == room_treasure).unwrap());
            }
        }

        // Try to assign Shop Room
        if !special.is_empty() && (n <= 15 || self.chance(66)) {
            let room_shop = self.choice(special.clone());
            board[room_shop.0 as usize][room_shop.1 as usize] = RoomType::Shop;

            special.remove(special.iter().position(|&x| x == room_shop).unwrap());
        }

        // Try to assign Devil or Angel Room
        let mut exist_devil_room = false;
        let mut exist_angel_room = false;

        if self.chance(20) {
            let mut y_min = 9;
            let mut y_max = 0;
            let mut x_min = 9;
            let mut x_max = 0;

            room.push((4, 4));

            for &(y, x) in room.iter() {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }

            let dy_reward: [i64; 4] = [0, 0, 1, -1];
            let dx_reward: [i64; 4] = [1, -1, 0, 0];
            let mut reward = Vec::new();

            for i in 0..4 {
                let y_next = room_boss.0 as i64 + dy_reward[i];
                let x_next = room_boss.1 as i64 + dx_reward[i];

                if !(y_next >= y_min && y_next <= y_max && x_next >= x_min && x_next <= x_max) {
                    continue;
                }

                let y_next = y_next as usize;
                let x_next = x_next as usize;

                if board[y_next][x_next] != RoomType::Empty {
                    continue;
                }

                reward.push((y_next, x_next));
            }

            let room_reward = self.choice(reward.clone());

            if self.chance(50) {
                exist_devil_room = true;
                board[room_reward.0 as usize][room_reward.1 as usize] = RoomType::Devil;
            } else {
                exist_angel_room = true;
                board[room_reward.0 as usize][room_reward.1 as usize] = RoomType::Angel;
            }
        }

        // Try to assign Sacrifice Room
        if !special.is_empty() && (exist_angel_room || self.chance(14)) {
            let room_sacrifice = self.choice(special.clone());
            board[room_sacrifice.0 as usize][room_sacrifice.1 as usize] = RoomType::Sacrifice;

            special.remove(special.iter().position(|&x| x == room_sacrifice).unwrap());
        }

        // Try to assign Curse Room
        if !special.is_empty() && exist_devil_room && self.chance(50) {
            let room_curse = self.choice(special.clone());
            board[room_curse.0 as usize][room_curse.1 as usize] = RoomType::Curse;

            special.remove(special.iter().position(|&x| x == room_curse).unwrap());
        }

        // Assign additional Normal Room
        for &(y, x) in special.iter() {
            board[y as usize][x as usize] = RoomType::Normal(self.choice(require.clone()));
        }

        for _ in 0..4 {
            let mut flag = true;

            while flag {
                for elem in board.last().unwrap().iter() {
                    if *elem != RoomType::Empty {
                        flag = false;
                        break;
                    }
                }

                if flag {
                    board.pop();
                }
            }

            board = Self::rotate_board(&board);
        }

        self.board = board;
    }

    fn process_dfs(
        &self,
        bit_node: &mut Vec<i64>,
        rooms_map: &BTreeMap<(usize, usize), usize>,
        y: usize,
        x: usize,
        bit: i64,
    ) {
        let curr = rooms_map[&(y, x)];
        bit_node[curr] = bit;

        let height = self.board.len();
        let width = self.board[0].len();
        let dy: [i64; 4] = [0, 1, 0, -1];
        let dx: [i64; 4] = [1, 0, -1, 0];

        for i in 0..4 {
            let y_next = y as i64 + dy[i];
            let x_next = x as i64 + dx[i];

            if y_next < 0 || y_next >= height as i64 || x_next < 0 || x_next >= width as i64 {
                continue;
            }

            let y_next = y_next as usize;
            let x_next = x_next as usize;

            if matches!(self.board[y_next][x_next], RoomType::Empty) {
                continue;
            }

            if matches!(
                self.board[y_next][x_next],
                RoomType::Devil | RoomType::Angel
            ) {
                continue;
            }

            let next = rooms_map[&(y_next, x_next)];

            if bit_node[next] != -1 {
                continue;
            }

            self.process_dfs(bit_node, rooms_map, y_next, x_next, bit | (1 << curr));
        }
    }

    fn process_backtracking(
        &self,
        set_shop: &mut BTreeSet<(i64, i64, i64, i64, i64)>,
        set_defeat: &mut BTreeSet<(i64, i64, i64, i64, i64)>,
        bit_node: &Vec<i64>,
        bit_end: &i64,
        rooms: &Vec<RoomType>,
        cnt_room: &usize,
        health: i64,
        attack: i64,
        coin: i64,
        bomb: i64,
        bit: i64,
    ) -> bool {
        let curr = (health, attack, coin, bomb, bit);

        if set_defeat.contains(&curr) {
            return false;
        }

        if health <= 0 {
            return false;
        }

        if coin < 0 || bomb < 0 {
            return false;
        }

        if (bit & bit_end) != 0 {
            return attack >= 10;
        }

        for next in 0..*cnt_room {
            if (bit & (1 << next)) != 0 {
                continue;
            }

            if (bit_node[next] & bit) != bit_node[next] {
                continue;
            }

            let room_type = rooms[next];
            let bit_next = bit | (1 << next);

            match room_type {
                RoomType::Empty | RoomType::Start | RoomType::Devil | RoomType::Angel => {}
                RoomType::Normal(require) => {
                    if attack >= require {
                        if self.process_backtracking(
                            set_shop,
                            set_defeat,
                            bit_node,
                            bit_end,
                            rooms,
                            cnt_room,
                            health,
                            attack,
                            coin + 1,
                            bomb,
                            bit_next,
                        ) {
                            return true;
                        }
                    } else {
                        if self.process_backtracking(
                            set_shop,
                            set_defeat,
                            bit_node,
                            bit_end,
                            rooms,
                            cnt_room,
                            health - 1,
                            attack,
                            coin + 1,
                            bomb,
                            bit_next,
                        ) {
                            return true;
                        }
                    }

                    if self.process_backtracking(
                        set_shop,
                        set_defeat,
                        bit_node,
                        bit_end,
                        rooms,
                        cnt_room,
                        health,
                        attack,
                        coin + 1,
                        bomb - 1,
                        bit_next,
                    ) {
                        return true;
                    }
                }
                RoomType::Boss => {
                    if self.process_backtracking(
                        set_shop, set_defeat, bit_node, bit_end, rooms, cnt_room, health, attack,
                        coin, bomb, bit_next,
                    ) {
                        return true;
                    }
                }
                RoomType::Secret => {
                    if self.process_backtracking(
                        set_shop,
                        set_defeat,
                        bit_node,
                        bit_end,
                        rooms,
                        cnt_room,
                        health + 2,
                        attack + 2,
                        coin + 2,
                        bomb - 1,
                        bit_next,
                    ) {
                        return true;
                    }
                }
                RoomType::Treasure => {
                    if self.process_backtracking(
                        set_shop,
                        set_defeat,
                        bit_node,
                        bit_end,
                        rooms,
                        cnt_room,
                        health,
                        attack + 1,
                        coin,
                        bomb,
                        bit_next,
                    ) {
                        return true;
                    }
                }
                RoomType::Shop => {
                    let potion_red = (health, attack + 1, coin - 2, bomb, bit);

                    if !set_shop.contains(&potion_red) {
                        set_shop.insert(potion_red);

                        if self.process_backtracking(
                            set_shop,
                            set_defeat,
                            bit_node,
                            bit_end,
                            rooms,
                            cnt_room,
                            health,
                            attack + 1,
                            coin - 2,
                            bomb,
                            bit,
                        ) {
                            return true;
                        }
                    }

                    let potion_blue = (health + 1, attack, coin - 2, bomb, bit);

                    if !set_shop.contains(&potion_blue) {
                        set_shop.insert(potion_blue);

                        if self.process_backtracking(
                            set_shop,
                            set_defeat,
                            bit_node,
                            bit_end,
                            rooms,
                            cnt_room,
                            health + 1,
                            attack,
                            coin - 2,
                            bomb,
                            bit,
                        ) {
                            return true;
                        }
                    }
                }
                RoomType::Sacrifice => {
                    if self.process_backtracking(
                        set_shop,
                        set_defeat,
                        bit_node,
                        bit_end,
                        rooms,
                        cnt_room,
                        health - 2,
                        attack + 3,
                        coin,
                        bomb,
                        bit_next,
                    ) {
                        return true;
                    }
                }
                RoomType::Curse => {
                    if self.process_backtracking(
                        set_shop,
                        set_defeat,
                        bit_node,
                        bit_end,
                        rooms,
                        cnt_room,
                        health,
                        attack - 2,
                        coin + 3,
                        bomb + 1,
                        bit_next,
                    ) {
                        return true;
                    }
                }
            }
        }

        set_defeat.insert(curr);

        false
    }

    fn process_game(&self) -> bool {
        let height = self.board.len();
        let width = self.board[0].len();

        // Construct TSP
        let mut y_start = 0;
        let mut x_start = 0;
        let mut y_end = 0;
        let mut x_end = 0;

        let mut rooms = Vec::new();
        let mut rooms_map = BTreeMap::new();
        let mut room_id = 0;

        for i in 0..height {
            for j in 0..width {
                if matches!(self.board[i][j], RoomType::Devil | RoomType::Angel) {
                    continue;
                }

                if !matches!(self.board[i][j], RoomType::Empty) {
                    rooms.push(self.board[i][j]);
                    rooms_map.insert((i, j), room_id);
                    room_id += 1;
                }

                if matches!(self.board[i][j], RoomType::Start) {
                    y_start = i;
                    x_start = j;
                } else if matches!(self.board[i][j], RoomType::Boss) {
                    y_end = i;
                    x_end = j;
                }
            }
        }

        // DFS
        let mut bit_node = vec![-1; room_id];
        let bit_end = 1 << rooms_map[&(y_end, x_end)];

        self.process_dfs(&mut bit_node, &rooms_map, y_start, x_start, 0);

        // Backtracking
        let mut set_shop = BTreeSet::new();
        let mut set_defeat = BTreeSet::new();

        let ret = self.process_backtracking(
            &mut set_shop,
            &mut set_defeat,
            &bit_node,
            &bit_end,
            &rooms,
            &room_id,
            6,
            1,
            0,
            3,
            1 << rooms_map[&(y_start, x_start)],
        );

        ret
    }

    fn rotate_board(board: &Vec<Vec<RoomType>>) -> Vec<Vec<RoomType>> {
        let height = board.len();
        let width = board[0].len();
        let mut ret = vec![vec![RoomType::Empty; height]; width];

        for i in 0..height {
            for j in 0..width {
                ret[j][height - i - 1] = board[i][j].clone();
            }
        }

        ret
    }

    fn print_room(
        board: &mut Vec<Vec<char>>,
        blueprint: &Vec<Vec<char>>,
        center_y: usize,
        center_x: usize,
    ) {
        let height = blueprint.len();
        let width = blueprint[0].len();

        for i in 0..height {
            for j in 0..width {
                if blueprint[i][j] == '*' {
                    continue;
                }

                board[6 * center_y + i + 2][6 * center_x + j + 2] = blueprint[i][j];
            }
        }
    }

    fn print_passage(
        board: &mut Vec<Vec<char>>,
        blueprint: &Vec<Vec<char>>,
        center_y: usize,
        center_x: usize,
    ) {
        for i in -1..=1 {
            for j in -1..=1 {
                if blueprint[(i + 1) as usize][(j + 1) as usize] == '!' {
                    board[(center_y as i64 + i) as usize][(center_x as i64 + j) as usize] = ' ';
                } else if board[(center_y as i64 + i) as usize][(center_x as i64 + j) as usize]
                    != '@'
                {
                    board[(center_y as i64 + i) as usize][(center_x as i64 + j) as usize] =
                        blueprint[(i + 1) as usize][(j + 1) as usize];
                }
            }
        }
    }

    fn print_board(&self, out: &mut BufWriter<StdoutLock>) {
        let height = self.board.len();
        let width = self.board[0].len();
        let height_total = 6 * height + 3;
        let width_total = 6 * width + 3;
        let mut ret = vec![vec![' '; width_total]; height_total];

        // Dungeon Boundary
        for i in 0..width_total {
            ret[0][i] = '#';
            ret[height_total - 1][i] = '#';
        }

        for i in 0..height_total {
            ret[i][0] = '#';
            ret[i][width_total - 1] = '#';
        }

        // Room Type
        for i in 0..height {
            for j in 0..width {
                ret[6 * i + 4][6 * j + 4] = match self.board[i][j] {
                    RoomType::Empty => ' ',
                    RoomType::Start => 'R',
                    RoomType::Normal(attack) => (attack as u8 + b'0') as char,
                    RoomType::Boss => 'B',
                    RoomType::Secret => 'X',
                    RoomType::Treasure => 'T',
                    RoomType::Shop => 'M',
                    RoomType::Devil => 'D',
                    RoomType::Angel => 'A',
                    RoomType::Sacrifice => 'S',
                    RoomType::Curse => 'C',
                };
            }
        }

        // Room Boundary
        let boundary_start_or_boss = vec![
            vec!['@', '@', '@', '@', '@'],
            vec!['@', ' ', ' ', ' ', '@'],
            vec!['@', ' ', '*', ' ', '@'],
            vec!['@', ' ', ' ', ' ', '@'],
            vec!['@', '@', '@', '@', '@'],
        ];
        let boundary_devil_or_angel = vec![
            vec!['/', '^', '^', '^', '\\'],
            vec!['<', ' ', ' ', ' ', '>'],
            vec!['<', ' ', '*', ' ', '>'],
            vec!['<', ' ', ' ', ' ', '>'],
            vec!['\\', 'v', 'v', 'v', '/'],
        ];
        let boundary_normal = vec![
            vec!['+', '-', '-', '-', '+'],
            vec!['|', ' ', ' ', ' ', '|'],
            vec!['|', ' ', '*', ' ', '|'],
            vec!['|', ' ', ' ', ' ', '|'],
            vec!['+', '-', '-', '-', '+'],
        ];

        for i in 0..height {
            for j in 0..width {
                if matches!(self.board[i][j], RoomType::Empty) {
                    continue;
                }

                if matches!(self.board[i][j], RoomType::Start | RoomType::Boss) {
                    BindingOfIsaac::print_room(&mut ret, &boundary_start_or_boss, i, j);
                } else if matches!(self.board[i][j], RoomType::Devil | RoomType::Angel) {
                    BindingOfIsaac::print_room(&mut ret, &boundary_devil_or_angel, i, j);
                } else {
                    BindingOfIsaac::print_room(&mut ret, &boundary_normal, i, j);
                }
            }
        }

        // Passage
        let passage_horizontal = vec![
            vec!['+', '-', '+'],
            vec!['!', ' ', '!'],
            vec!['+', '-', '+'],
        ];
        let passage_vertical = vec![
            vec!['+', '!', '+'],
            vec!['|', ' ', '|'],
            vec!['+', '!', '+'],
        ];

        // Passage - Horizontal
        for i in (4..height_total).step_by(6) {
            for j in (1..width_total).step_by(6) {
                let mut is_satisfied = true;

                if j as i64 + 3 < width_total as i64
                    && (ret[i][j + 3] == 'X' || ret[i][j + 3] == 'D' || ret[i][j + 3] == 'A')
                {
                    is_satisfied = false;
                }

                if j as i64 - 3 >= 0
                    && (ret[i][j - 3] == 'X' || ret[i][j - 3] == 'D' || ret[i][j - 3] == 'A')
                {
                    is_satisfied = false;
                }

                if !(ret[i][j - 1] == '|' || ret[i][j - 1] == '@') {
                    is_satisfied = false;
                }

                if !(ret[i][j + 1] == '|' || ret[i][j + 1] == '@') {
                    is_satisfied = false;
                }

                if !is_satisfied {
                    continue;
                }

                BindingOfIsaac::print_passage(&mut ret, &passage_horizontal, i, j);
            }
        }

        // Passage - Vertical
        for j in (4..width_total).step_by(6) {
            for i in (1..height_total).step_by(6) {
                let mut is_satisfied = true;

                if i as i64 + 3 < height_total as i64
                    && (ret[i + 3][j] == 'X' || ret[i + 3][j] == 'D' || ret[i + 3][j] == 'A')
                {
                    is_satisfied = false;
                }

                if i as i64 - 3 >= 0
                    && (ret[i - 3][j] == 'X' || ret[i - 3][j] == 'D' || ret[i - 3][j] == 'A')
                {
                    is_satisfied = false;
                }

                if !(ret[i - 1][j] == '-' || ret[i - 1][j] == '@') {
                    is_satisfied = false;
                }

                if !(ret[i + 1][j] == '-' || ret[i + 1][j] == '@') {
                    is_satisfied = false;
                }

                if !is_satisfied {
                    continue;
                }

                if !is_satisfied {
                    continue;
                }

                BindingOfIsaac::print_passage(&mut ret, &passage_vertical, i, j);
            }
        }

        // Print result
        for i in 0..height_total {
            for j in 0..width_total {
                write!(out, "{}", ret[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let seed = scan.token::<String>();
    let mut game = BindingOfIsaac::new(seed);

    game.generate_dungeon();
    let ret = game.process_game();

    writeln!(out, "{}", if ret { "CLEAR" } else { "GAME OVER" }).unwrap();

    game.print_board(&mut out);
}
