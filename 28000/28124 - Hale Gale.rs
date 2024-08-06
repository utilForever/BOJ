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

#[derive(Clone, Debug)]
struct Player {
    left: usize,
    right: usize,
    round: i64,
    card_picture: i64,
    card_number: i64,
    remain_cards: i64,
}

impl Player {
    fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            round: 0,
            card_picture: 0,
            card_number: 0,
            remain_cards: 0,
        }
    }

    fn is_dead(&self) -> bool {
        self.remain_cards == 0
    }
}

#[derive(Clone, Debug)]
struct Card {
    round: i64,
    checksum: i64,
    total: i64,
}

impl Card {
    fn new() -> Self {
        Self {
            round: 0,
            checksum: -1,
            total: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    cards: Vec<Card>,

    round: i64,
    cur_player: usize,
    players_alive_round: i64,
    players_alive_total: i64,
    count_hale: i64,
    count_gale: i64,
}

impl Game {
    fn new(players: Vec<Player>, cards: Vec<Card>, num_players: i64) -> Self {
        Self {
            players,
            cards,
            round: 0,
            cur_player: 1,
            players_alive_round: num_players,
            players_alive_total: num_players,
            count_hale: 0,
            count_gale: 0,
        }
    }

    fn start_new(&mut self) {
        self.round += 1;
        self.cur_player = self.players[self.cur_player].left;
        self.players_alive_round = self.players_alive_total;
        self.count_hale = 0;
        self.count_gale = 0;
    }

    fn process_turn(&mut self, idx: i64, x: i64) {
        self.cur_player = self.players[self.cur_player].right;

        while self.players[self.cur_player].is_dead() {
            self.cur_player = self.players[self.cur_player].right;
        }

        self.process_hale(idx, x);
        self.check_cur_player(self.cur_player);
    }

    fn process_hale(&mut self, idx: i64, x: i64) {
        if self.round != self.players[self.cur_player].round {
            self.players[self.cur_player].round = self.round;
            self.players[self.cur_player].card_picture = 0;
            self.players[self.cur_player].card_number = 0;
        }

        self.count_hale += 1;
        self.adjust_card(-(self.players[self.cur_player].card_number));

        self.players[self.cur_player].card_picture = idx;
        self.adjust_card(x);
        self.players[self.cur_player].card_number = x;
        self.players[self.cur_player].remain_cards -= 1;
    }

    fn adjust_card(&mut self, count: i64) {
        let card = &mut self.cards[self.players[self.cur_player].card_picture as usize];

        if card.round != self.round {
            card.round = self.round;
            card.total = 0;
        }

        if card.checksum == card.total {
            self.count_gale -= 1;
        }

        card.total += count;

        if card.checksum == card.total {
            self.count_gale += 1;
        }
    }

    fn check_cur_player(&mut self, idx: usize) {
        if self.players[idx].remain_cards <= 0 {
            let left = self.players[idx].left;
            let right = self.players[idx].right;

            self.players[left].right = self.players[idx].right;
            self.players[right].left = self.players[idx].left;
            self.players[idx].remain_cards = 0;
            self.players_alive_total -= 1;
        }
    }

    fn process_gale(&mut self, p: usize) {
        if self.count_gale > 0 {
            self.cur_player = p;
            self.players[self.cur_player].remain_cards += self.count_hale;

            if self.players[self.cur_player].remain_cards == self.count_hale {
                self.revive_player();
            }

            self.start_new();
        } else {
            let player = &mut self.players[p];

            if self.round != player.round {
                player.round = self.round;
                player.card_picture = 0;
                player.card_number = 0;
            }

            if player.is_dead() {
                return;
            }

            player.remain_cards -= self.players_alive_round;
            self.check_cur_player(p);
        }
    }

    fn revive_player(&mut self) {
        self.players_alive_total += 1;

        if self.players_alive_total == 1 {
            self.players[self.cur_player].left = self.cur_player;
            self.players[self.cur_player].right = self.cur_player;
        } else {
            let mut left = self.players[self.cur_player].left;

            while self.players[left].is_dead() {
                left = self.players[left].left;
            }

            let mut right = self.players[self.cur_player].right;

            while self.players[right].is_dead() {
                right = self.players[right].right;
            }

            self.players[left].right = self.cur_player;
            self.players[right].left = self.cur_player;
            self.players[self.cur_player].left = left;
            self.players[self.cur_player].right = right;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (p, n, k, h, g) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut players = vec![Player::new(); p + 1];
    let mut cards = vec![Card::new(); k + 1];

    for i in 1..=p {
        players[i].left = if i == 1 { p } else { i - 1 };
        players[i].right = if i == p { 1 } else { i + 1 };
        players[i].remain_cards = n;
    }

    for i in 1..=k {
        cards[i].checksum = scan.token::<i64>();
    }

    let mut game = Game::new(players, cards, p as i64);
    game.start_new();

    for _ in 0..h + g {
        let idx = scan.token::<i64>();

        if idx > 0 {
            // Hale
            let x = scan.token::<i64>();
            game.process_turn(idx, x);
        } else {
            // Gale
            let p = scan.token::<usize>();
            game.process_gale(p);
        }
    }

    // Print result
    writeln!(out, "{}", game.players_alive_round).unwrap();

    let cur_player = game.cur_player;

    for i in 0..p {
        let player = &game.players[(cur_player + i) % p + 1];

        if player.is_dead() && player.round < game.round {
            continue;
        }

        write!(out, "{} ", player.remain_cards).unwrap();

        if player.round == game.round {
            writeln!(out, "{} {}", player.card_picture, player.card_number).unwrap();
        } else {
            writeln!(out, "0 0").unwrap();
        }
    }
}
