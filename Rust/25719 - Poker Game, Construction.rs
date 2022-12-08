use io::Write;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{BTreeMap, HashMap},
    io, str,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "T",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            }
        )
    }
}

impl Rank {
    fn from_i32(i: i32) -> Rank {
        match i {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Suit {
    Diamond = 1,
    Club = 2,
    Heart = 3,
    Spade = 4,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Diamond => "D",
                Suit::Club => "C",
                Suit::Heart => "H",
                Suit::Spade => "S",
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let convert_card = |raw_card: &str| -> Card {
        let suit = match &raw_card[1..2] {
            "D" => Suit::Diamond,
            "C" => Suit::Club,
            "H" => Suit::Heart,
            "S" => Suit::Spade,
            _ => panic!("Invalid suit"),
        };
        let rank = match &raw_card[0..1] {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "T" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Invalid rank"),
        };

        Card { rank, suit }
    };

    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    let suits = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut cards_alice = Vec::new();
        let mut cards_bob = Vec::new();

        for _ in 0..2 {
            cards_alice.push(convert_card(&scan.token::<String>()));
        }

        for _ in 0..2 {
            cards_bob.push(convert_card(&scan.token::<String>()));
        }

        cards_alice.sort_by(|a, b| b.rank.cmp(&a.rank));
        cards_bob.sort_by(|a, b| b.rank.cmp(&a.rank));

        let cards = cards_alice
            .clone()
            .into_iter()
            .chain(cards_bob.clone().into_iter())
            .collect::<Vec<_>>();
        let cnt_ranks = cards
            .iter()
            .fold(HashMap::new(), |mut acc, card| {
                *acc.entry(card.rank).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let card_map = cards
            .iter()
            .fold(HashMap::<Rank, Vec<Suit>>::new(), |mut acc, card| {
                acc.entry(card.rank).or_default().push(card.suit);
                acc
            })
            .into_iter()
            .collect::<BTreeMap<_, _>>();

        // How to make Alice win
        write!(out, "YES ").unwrap();

        if cards_alice[0].rank == cards_alice[1].rank {
            if cnt_ranks[&cards_alice[0].rank] == 2 {
                for suit in suits.iter() {
                    if card_map[&cards_alice[0].rank].contains(suit) {
                        continue;
                    }

                    write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
                }

                // Random different rank cards
                for rank in ranks.iter() {
                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    for suit in suits.iter() {
                        write!(out, "{}{} ", rank, suit).unwrap();
                    }

                    break;
                }

                writeln!(out).unwrap();
            } else {
                let mut cnt = 0;

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();
                }

                writeln!(out).unwrap();
            }
        } else if cards_bob[0].rank == cards_bob[1].rank {
            let mut cnt = 0;

            if cards_alice[0].rank == cards_bob[0].rank {
                for suit in suits.iter() {
                    if card_map[&cards_alice[1].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
                }

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    cnt += 1;

                    write!(
                        out,
                        "{}{} ",
                        rank,
                        suits[(cards_bob[0].suit as usize + 1) % 4]
                    )
                    .unwrap();
                }

                writeln!(out).unwrap();
            } else if cards_alice[1].rank == cards_bob[0].rank {
                for suit in suits.iter() {
                    if card_map[&cards_alice[0].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
                }

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    cnt += 1;

                    write!(
                        out,
                        "{}{} ",
                        rank,
                        suits[(cards_bob[0].suit as usize + 1) % 4]
                    )
                    .unwrap();
                }

                writeln!(out).unwrap();
            } else {
                let mut cnt = 0;

                for suit in suits.iter() {
                    if card_map[&cards_alice[0].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
                }

                for suit in suits.iter() {
                    if card_map[&cards_alice[1].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
                }

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    cnt += 1;

                    write!(
                        out,
                        "{}{} ",
                        rank,
                        suits[(cards_bob[0].suit as usize + 1) % 4]
                    )
                    .unwrap();
                }

                writeln!(out).unwrap();
            }
        } else {
            let mut cnt = 0;

            for suit in suits.iter() {
                if card_map[&cards_alice[0].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
            }

            for suit in suits.iter() {
                if card_map[&cards_alice[1].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
            }

            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                cnt += 1;

                write!(
                    out,
                    "{}{} ",
                    rank,
                    suits[(cards_bob[0].suit as usize + 1) % 4]
                )
                .unwrap();
            }

            writeln!(out).unwrap();
        }

        // How to make Bob win
        let is_bob_cards_unique_ranks = cnt_ranks.contains_key(&cards_bob[0].rank)
            && cnt_ranks.contains_key(&cards_bob[1].rank)
            && cnt_ranks[&cards_bob[0].rank] == 1
            && cnt_ranks[&cards_bob[1].rank] == 1;

        if cards_bob[0].rank == cards_bob[1].rank {
            if cnt_ranks[&cards_bob[0].rank] == 4 {
                // Case XX vs XX
                writeln!(out, "NO").unwrap();
            } else if cards_alice[0].rank == cards_alice[1].rank {
                // Case XX vs YY
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                // Print Y cards with different suit
                for suit in suits.iter() {
                    if card_map[&cards_bob[0].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_bob[0].rank, suit).unwrap();
                }

                // Print different rank cards except X and Y
                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            continue;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, suit).unwrap();
                    }
                }

                writeln!(out).unwrap();
            } else {
                // Case XY vs ZZ
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                // Print different rank cards except X, Y and Z
                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            continue;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, suit).unwrap();
                    }
                }

                writeln!(out).unwrap();
            }
        } else if is_bob_cards_unique_ranks {
            write!(out, "YES ").unwrap();

            for suit in suits.iter() {
                if card_map[&cards_bob[0].rank].contains(suit) {
                    continue;
                }

                write!(out, "{}{} ", cards_bob[0].rank, suit).unwrap();
            }

            for suit in suits.iter() {
                if card_map[&cards_bob[1].rank].contains(suit) {
                    continue;
                }

                write!(out, "{}{} ", cards_bob[1].rank, suit).unwrap();
            }

            writeln!(out).unwrap();
        } else if cards_bob[0].suit == cards_bob[1].suit {
            // If Two bob cards have same suit, can win by flush
            write!(out, "YES ").unwrap();

            let mut cnt = 0;

            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank, cards_bob[0].suit).unwrap();
            }

            writeln!(out).unwrap();
        } else if cards_alice[0].rank == cards_alice[1].rank {
            // Case AA vs AX
            let cond1 = cards_alice[0].rank == Rank::Ace
                && cards_bob[0].rank == Rank::Ace
                && cards_bob[1].rank != Rank::Ace;
            // Case KK vs AK
            let cond2 = cards_bob[0].rank == Rank::Ace
                && cards_alice[0].rank == Rank::King
                && cards_bob[1].rank == Rank::King
                && cards_alice[0].rank == cards_bob[1].rank;

            if cond1 || cond2 {
                if cards_bob[1].rank >= Rank::Six && cards_bob[1].rank <= Rank::Nine {
                    // If X is 6, 7, 8 or 9, can't win
                    writeln!(out, "NO").unwrap();
                } else if cards_bob[1].rank >= Rank::Ten && cards_bob[1].rank <= Rank::King {
                    // If X is 10, J, Q or K, can win by straight
                    write!(out, "YES ").unwrap();

                    for rank in [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King].iter() {
                        if cards_bob[1].rank == *rank {
                            continue;
                        }

                        write!(
                            out,
                            "{}{} ",
                            rank,
                            suits[(cards_alice[0].suit as usize + 1) % 4]
                        )
                        .unwrap();
                        write!(
                            out,
                            "{}{} ",
                            rank,
                            suits[(cards_alice[0].suit as usize + 2) % 4]
                        )
                        .unwrap();
                    }

                    writeln!(out).unwrap();
                } else {
                    // If X is 2, 3, 4 or 5, can win by straight
                    write!(out, "YES ").unwrap();

                    for rank in [Rank::Two, Rank::Three, Rank::Four, Rank::Five].iter() {
                        if cards_bob[1].rank == *rank {
                            continue;
                        }

                        write!(
                            out,
                            "{}{} ",
                            rank,
                            suits[(cards_alice[0].suit as usize + 1) % 4]
                        )
                        .unwrap();
                        write!(
                            out,
                            "{}{} ",
                            rank,
                            suits[(cards_alice[0].suit as usize + 2) % 4]
                        )
                        .unwrap();
                    }

                    writeln!(out).unwrap();
                }
            } else {
                // Case XX vs YZ
                write!(out, "YES ").unwrap();

                let mut rank_bob1 = cards_bob[0].rank;
                let mut rank_bob2 = cards_bob[1].rank;

                // Check X and Y are same rank
                if cards_alice[0].rank == rank_bob1 {
                    std::mem::swap(&mut rank_bob1, &mut rank_bob2);
                }

                // Print 3 cards with same rank as rank_bob1
                for suit in suits.iter() {
                    if card_map[&rank_bob1].contains(suit) {
                        continue;
                    }

                    write!(out, "{}{} ", rank_bob1, suit).unwrap();
                }

                // Get new rank that different from X, Y and Z
                let rank_new = if cnt_ranks.contains_key(&Rank::Ace) {
                    Rank::from_i32(rank_bob2 as i32 + 1)
                } else {
                    Rank::Ace
                };

                // Print rest 3 cards with same rank as rank_new
                for (cnt, suit) in suits.iter().enumerate() {
                    if cnt == 3 {
                        break;
                    }

                    if card_map.contains_key(&rank_new) && card_map[&rank_new].contains(suit) {
                        continue;
                    }

                    write!(out, "{}{} ", rank_new, suit).unwrap();
                }

                writeln!(out).unwrap();
            }
        } else if cards_bob[0].rank != cards_bob[1].rank
            && cards_bob[0].rank != cards_alice[0].rank
            && cards_bob[0].rank != cards_alice[1].rank
        {
            write!(out, "YES ").unwrap();

            let mut cnt = 0;

            for suit in suits.iter() {
                if cnt == 2 {
                    break;
                }

                if card_map[&cards_bob[0].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_bob[0].rank, suit).unwrap();
            }

            for suit in suits.iter() {
                if card_map[&cards_bob[1].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_bob[1].rank, suit).unwrap();
            }

            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();
            }

            writeln!(out).unwrap();
        } else if cards_bob[1].rank != cards_bob[0].rank
            && cards_bob[1].rank != cards_alice[0].rank
            && cards_bob[1].rank != cards_alice[1].rank
        {
            write!(out, "YES ").unwrap();

            let mut cnt = 0;

            for suit in suits.iter() {
                if cnt == 2 {
                    break;
                }

                if card_map[&cards_bob[1].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_bob[1].rank, suit).unwrap();
            }

            for suit in suits.iter() {
                if card_map[&cards_bob[0].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_bob[0].rank, suit).unwrap();
            }

            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
            }

            writeln!(out).unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }

        // How to make the game in a draw
        if cards_alice[0].rank == cards_alice[1].rank && cards_bob[0].rank == cards_bob[1].rank {
            if cards_alice[0].rank != cards_bob[0].rank {
                writeln!(out, "NO").unwrap();
            } else {
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            break;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, suit).unwrap();
                    }
                }

                writeln!(out).unwrap();
            }
        } else if cards_alice[0].rank == cards_alice[1].rank
            && cards_bob[0].rank != cards_bob[1].rank
        {
            // Can draw! For example, 3C 3D 5S 3H
            // Solution: 3S 5D 5C 5H 2D 2C
            if cards_alice[0].rank < cards_bob[0].rank && cards_alice[0].rank == cards_bob[1].rank {
                write!(out, "YES ").unwrap();

                let mut vec_rank = vec![Vec::new(); 15];

                for card in cards_alice.iter() {
                    vec_rank[card.rank as usize].push(card.suit);
                }

                for card in cards_bob.iter() {
                    vec_rank[card.rank as usize].push(card.suit);
                }

                let mut cnt = 0;

                for i in 2..=14 {
                    if cnt == 6 {
                        break;
                    }

                    if vec_rank[i].is_empty() {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            break;
                        }

                        if vec_rank[i].contains(suit) {
                            continue;
                        }

                        write!(out, "{}{} ", ranks[i - 2], suit).unwrap();
                        cnt += 1;
                    }
                }

                // Random two same rank cards
                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();
                    write!(out, "{}{} ", rank, suits[(cnt + 2) % 4]).unwrap();

                    cnt += 2;
                }

                writeln!(out).unwrap();
            } else {
                // How about 5C 5D 5S 3H? Can't draw
                writeln!(out, "NO").unwrap();
            }
        } else if cards_alice[0].rank != cards_alice[1].rank
            && cards_bob[0].rank == cards_bob[1].rank
        {
            if cards_alice[0].rank == cards_bob[0].rank {
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                for suit in suits.iter() {
                    if cnt == 1 {
                        break;
                    }

                    if card_map[&cards_alice[1].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
                }

                for suit in suits.iter() {
                    if cnt == 2 {
                        break;
                    }

                    if card_map[&cards_alice[0].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
                }

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            break;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, suit).unwrap();
                    }
                }

                writeln!(out).unwrap();
            } else if cards_alice[1].rank == cards_bob[0].rank {
                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                for suit in suits.iter() {
                    if cnt == 1 {
                        break;
                    }

                    if card_map[&cards_alice[0].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
                }

                for suit in suits.iter() {
                    if cnt == 2 {
                        break;
                    }

                    if card_map[&cards_alice[1].rank].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
                }

                for rank in ranks.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if cnt_ranks.contains_key(rank) {
                        continue;
                    }

                    for suit in suits.iter() {
                        if cnt == 6 {
                            break;
                        }

                        cnt += 1;

                        write!(out, "{}{} ", rank, suit).unwrap();
                    }
                }

                writeln!(out).unwrap();
            } else {
                writeln!(out, "NO").unwrap();
            }
        } else if cnt_ranks[&cards_alice[0].rank] == 2 && cnt_ranks[&cards_alice[1].rank] == 2 {
            write!(out, "YES ").unwrap();

            let mut cnt = 0;

            // Random two same rank cards
            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();

                cnt += 2;
            }

            writeln!(out).unwrap();
        } else if cnt_ranks[&cards_alice[0].rank] == 2 {
            write!(out, "YES ").unwrap();

            let mut cnt = 0;
            let mut rank_new = Rank::Ace;

            if cards_bob[0].rank != cards_alice[0].rank {
                rank_new = cards_bob[0].rank;
            }
            if cards_bob[1].rank != cards_alice[0].rank {
                rank_new = cards_bob[1].rank;
            }

            for suit in suits.iter() {
                if card_map.contains_key(&rank_new) && card_map[&rank_new].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank_new, suit).unwrap();
            }

            for suit in suits.iter() {
                if cnt == 4 {
                    break;
                }

                if card_map[&cards_alice[1].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_alice[1].rank, suit).unwrap();
            }

            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();

                cnt += 2;
            }

            writeln!(out).unwrap();
        } else if cnt_ranks[&cards_alice[1].rank] == 2 {
            write!(out, "YES ").unwrap();

            let mut cnt = 0;
            let mut rank_new = Rank::Ace;

            if cards_bob[0].rank != cards_alice[1].rank {
                rank_new = cards_bob[0].rank;
            }
            if cards_bob[1].rank != cards_alice[1].rank {
                rank_new = cards_bob[1].rank;
            }

            for suit in suits.iter() {
                if card_map.contains_key(&rank_new) && card_map[&rank_new].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", rank_new, suit).unwrap();
            }

            for suit in suits.iter() {
                if cnt == 4 {
                    break;
                }

                if card_map[&cards_alice[0].rank].contains(suit) {
                    continue;
                }

                cnt += 1;

                write!(out, "{}{} ", cards_alice[0].rank, suit).unwrap();
            }

            for rank in ranks.iter() {
                if cnt == 6 {
                    break;
                }

                if cnt_ranks.contains_key(rank) {
                    continue;
                }

                write!(out, "{}{} ", rank, suits[cnt % 4]).unwrap();
                write!(out, "{}{} ", rank, suits[(cnt + 1) % 4]).unwrap();

                cnt += 2;
            }

            writeln!(out).unwrap();
        } else {
            let rank_alice1 = cards_alice[0].rank;
            let rank_alice2 = cards_alice[1].rank;
            let mut rank_bob1 = cards_bob[0].rank;
            let mut rank_bob2 = cards_bob[1].rank;

            if (rank_alice2 > rank_bob1 || rank_bob2 > rank_alice1)
                || (rank_bob2 < rank_alice2 && rank_bob1 > rank_alice1)
            {
                writeln!(out, "NO").unwrap();
            } else {
                if rank_bob2 < rank_alice1 && rank_bob2 > rank_alice2 {
                    std::mem::swap(&mut rank_bob1, &mut rank_bob2);
                }

                write!(out, "YES ").unwrap();

                let mut cnt = 0;

                for suit in suits.iter() {
                    if card_map[&rank_bob2].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank_bob2, suit).unwrap();
                }

                for suit in suits.iter() {
                    if cnt == 4 {
                        break;
                    }

                    if card_map[&rank_alice1].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank_alice1, suit).unwrap();
                }

                for suit in suits.iter() {
                    if cnt == 5 {
                        break;
                    }

                    if card_map[&rank_alice2].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank_alice2, suit).unwrap();
                }

                for suit in suits.iter() {
                    if cnt == 6 {
                        break;
                    }

                    if card_map[&rank_bob1].contains(suit) {
                        continue;
                    }

                    cnt += 1;

                    write!(out, "{}{} ", rank_bob1, suit).unwrap();
                }

                writeln!(out).unwrap();
            }
        }
    }
}
