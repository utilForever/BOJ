use io::Write;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::HashMap,
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

#[derive(Debug, Clone, Eq, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
enum Category {
    HighCard(Hand),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Hand),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(mut cards: Vec<Card>) -> Self {
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        Self { cards }
    }

    fn get_category(&self) -> Category {
        let is_suit_all_same = self
            .cards
            .iter()
            .all(|card| card.suit == self.cards[0].suit);
        let is_straight_normal = self
            .cards
            .windows(2)
            .all(|window| window[0].rank as i64 == window[1].rank as i64 + 1);
        let is_straight_baby = self.cards[0].rank == Rank::Ace
            && self.cards[1].rank == Rank::Five
            && self.cards[2].rank == Rank::Four
            && self.cards[3].rank == Rank::Three
            && self.cards[4].rank == Rank::Two;
        let is_straight = is_straight_normal || is_straight_baby;

        let ranks: HashMap<Rank, i64> = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card.rank).or_insert(0) += 1;
            acc
        });
        let mut ranks = ranks
            .iter()
            .map(|(rank, count)| (*rank, *count))
            .collect::<Vec<(Rank, i64)>>();
        ranks.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

        // Check straight flush
        if is_suit_all_same && is_straight {
            if self.cards[4].rank == Rank::Ten {
                return Category::RoyalFlush;
            } else if is_straight_baby {
                return Category::StraightFlush(Rank::Five);
            } else {
                return Category::StraightFlush(self.cards[0].rank);
            }
        }

        // Check four of a kind
        if ranks[0].1 == 4 {
            return Category::FourOfAKind(ranks[0].0, ranks[1].0);
        }

        // Check full house
        if ranks[0].1 == 3 && ranks[1].1 == 2 {
            return Category::FullHouse(ranks[0].0, ranks[1].0);
        }

        // Check flush
        if is_suit_all_same {
            return Category::Flush(self.clone());
        }

        // Check straight
        if is_straight {
            if is_straight_baby {
                return Category::Straight(ranks[1].0);
            } else {
                return Category::Straight(ranks[0].0);
            }
        }

        // Check three of a kind
        if ranks[0].1 == 3 {
            return Category::ThreeOfAKind(ranks[0].0, ranks[1].0, ranks[2].0);
        }

        // Check two pair
        if ranks[0].1 == 2 && ranks[1].1 == 2 {
            return Category::TwoPair(ranks[0].0, ranks[1].0, ranks[2].0);
        }

        // Check one pair
        if ranks[0].1 == 2 {
            return Category::OnePair(ranks[0].0, ranks[1].0, ranks[2].0, ranks[3].0);
        }

        // Check high card
        Category::HighCard(self.clone())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let convert = |category: Category| -> u8 {
            match category {
                Category::HighCard(_) => 1,
                Category::OnePair(_, _, _, _) => 2,
                Category::TwoPair(_, _, _) => 3,
                Category::ThreeOfAKind(_, _, _) => 4,
                Category::Straight(_) => 5,
                Category::Flush(_) => 6,
                Category::FullHouse(_, _) => 7,
                Category::FourOfAKind(_, _) => 8,
                Category::StraightFlush(_) => 9,
                Category::RoyalFlush => 10,
            }
        };

        let category = self.get_category();
        let other_category = other.get_category();
        let ret_compare = convert(category.clone()).cmp(&convert(other_category.clone()));

        match ret_compare {
            Ordering::Less | Ordering::Greater => Some(ret_compare),
            Ordering::Equal => {
                let ret = match (category, other_category) {
                    (Category::HighCard(hand), Category::HighCard(other_hand)) => {
                        Some(hand.cards.cmp(&other_hand.cards))
                    }
                    (
                        Category::OnePair(rank1, rank2, rank3, rank4),
                        Category::OnePair(other_rank1, other_rank2, other_rank3, other_rank4),
                    ) => Some(
                        rank1.cmp(&other_rank1).then(
                            rank2
                                .cmp(&other_rank2)
                                .then(rank3.cmp(&other_rank3).then(rank4.cmp(&other_rank4))),
                        ),
                    ),
                    (
                        Category::TwoPair(rank1, rank2, rank3),
                        Category::TwoPair(other_rank1, other_rank2, other_rank3),
                    ) => Some(
                        rank1
                            .cmp(&other_rank1)
                            .then(rank2.cmp(&other_rank2).then(rank3.cmp(&other_rank3))),
                    ),
                    (
                        Category::ThreeOfAKind(rank1, rank2, rank3),
                        Category::ThreeOfAKind(other_rank1, other_rank2, other_rank3),
                    ) => Some(
                        rank1
                            .cmp(&other_rank1)
                            .then(rank2.cmp(&other_rank2).then(rank3.cmp(&other_rank3))),
                    ),
                    (Category::Straight(rank), Category::Straight(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    (Category::Flush(hand), Category::Flush(other_hand)) => {
                        Some(hand.cards.cmp(&other_hand.cards))
                    }
                    (
                        Category::FullHouse(rank1, rank2),
                        Category::FullHouse(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (
                        Category::FourOfAKind(rank1, rank2),
                        Category::FourOfAKind(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (Category::StraightFlush(rank), Category::StraightFlush(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    (Category::RoyalFlush, Category::RoyalFlush) => Some(Ordering::Equal),
                    _ => None,
                };

                ret.filter(|compare| matches!(compare, Ordering::Less | Ordering::Greater))
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    // Precompute
    let mut hands_all = Vec::with_capacity(7462);

    // Royal flush
    hands_all.push((14, 13, 12, 11, 10));

    // Straight flush
    for i in (5..=13).rev() {
        hands_all.push((i, i - 1, i - 2, i - 3, i - 4));
    }

    // Four of a kind
    for i in (2..=14).rev() {
        for j in (2..=14).rev() {
            if i == j {
                continue;
            }

            hands_all.push((i, i, i, i, j));
        }
    }

    // Full house
    for i in (2..=14).rev() {
        for j in (2..=14).rev() {
            if i == j {
                continue;
            }

            hands_all.push((i, i, i, j, j));
        }
    }

    // Flush
    for i in (2..=14).rev() {
        for j in (2..=i - 1).rev() {
            for k in (2..=j - 1).rev() {
                for l in (2..=k - 1).rev() {
                    for m in (2..=l - 1).rev() {
                        // Ignore straight flush
                        if i - m == 4 {
                            continue;
                        }

                        // Ignore royal flush
                        if i == 14 && j == 5 && k == 4 && l == 3 && m == 2 {
                            continue;
                        }

                        hands_all.push((i, j, k, l, m));
                    }
                }
            }
        }
    }

    // Straight
    for i in (5..=14).rev() {
        hands_all.push((i, i - 1, i - 2, i - 3, i - 4));
    }

    // Three of a kind
    for i in (2..=14).rev() {
        for j in (2..=14).rev() {
            if i == j {
                continue;
            }

            for k in (2..=j - 1).rev() {
                if i == k {
                    continue;
                }

                hands_all.push((i, i, i, j, k));
            }
        }
    }

    // Two pair
    for i in (2..=14).rev() {
        for j in (2..=i - 1).rev() {
            for k in (2..=14).rev() {
                if i == k || j == k {
                    continue;
                }

                hands_all.push((i, i, j, j, k));
            }
        }
    }

    // One pair
    for i in (2..=14).rev() {
        for j in (2..=14).rev() {
            for k in (2..=j - 1).rev() {
                for l in (2..=k - 1).rev() {
                    if i == j || i == k || i == l {
                        continue;
                    }

                    hands_all.push((i, i, j, k, l));
                }
            }
        }
    }

    // High card
    for i in (2..=14).rev() {
        for j in (2..=i - 1).rev() {
            for k in (2..=j - 1).rev() {
                for l in (2..=k - 1).rev() {
                    for m in (2..=l - 1).rev() {
                        // Ignore straight
                        if i - m == 4 {
                            continue;
                        }

                        // Ignore royal flush
                        if i == 14 && j == 5 && k == 4 && l == 3 && m == 2 {
                            continue;
                        }

                        hands_all.push((i, j, k, l, m));
                    }
                }
            }
        }
    }

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

    let mut cards_raw_player = Vec::new();

    for _ in 0..5 {
        cards_raw_player.push(scan.token::<String>());
    }

    let mut cards_player = Vec::new();

    for i in 0..5 {
        cards_player.push(convert_card(&cards_raw_player[i]));
    }

    cards_player.sort();

    let hand_player = Hand::new(
        [
            cards_player[0].clone(),
            cards_player[1].clone(),
            cards_player[2].clone(),
            cards_player[3].clone(),
            cards_player[4].clone(),
        ]
        .to_vec(),
    );

    let category = hand_player.get_category();

    let idx_royal_flush = 1;
    let idx_straight_flush = 2;
    let idx_four_of_a_kind = 11;
    let idx_full_house = 167;
    let idx_flush = 323;
    let idx_straight = 1600;
    let idx_three_of_a_kind = 1610;
    let idx_two_pair = 2468;
    let idx_one_pair = 3326;
    let idx_high_card = 6186;

    let ret = match category {
        Category::RoyalFlush => idx_royal_flush,
        Category::StraightFlush(rank) => {
            idx_straight_flush
                + hands_all[idx_straight_flush - 1..]
                    .iter()
                    .position(|&x| x.0 == rank as i64)
                    .unwrap()
        }
        Category::FourOfAKind(rank1, rank2) => {
            idx_four_of_a_kind
                + hands_all[idx_four_of_a_kind - 1..]
                    .iter()
                    .position(|&x| x.0 == rank1 as i64 && x.4 == rank2 as i64)
                    .unwrap()
        }
        Category::FullHouse(rank1, rank2) => {
            idx_full_house
                + hands_all[idx_full_house - 1..]
                    .iter()
                    .position(|&x| x.0 == rank1 as i64 && x.3 == rank2 as i64)
                    .unwrap()
        }
        Category::Flush(hand) => {
            let rank1 = hand.cards[0].rank;
            let rank2 = hand.cards[1].rank;
            let rank3 = hand.cards[2].rank;
            let rank4 = hand.cards[3].rank;
            let rank5 = hand.cards[4].rank;

            idx_flush
                + hands_all[idx_flush - 1..]
                    .iter()
                    .position(|&x| {
                        x.0 == rank1 as i64
                            && x.1 == rank2 as i64
                            && x.2 == rank3 as i64
                            && x.3 == rank4 as i64
                            && x.4 == rank5 as i64
                    })
                    .unwrap()
        }
        Category::Straight(rank) => {
            idx_straight
                + hands_all[idx_straight - 1..]
                    .iter()
                    .position(|&x| x.0 == rank as i64)
                    .unwrap()
        }
        Category::ThreeOfAKind(rank1, rank2, rank3) => {
            idx_three_of_a_kind
                + hands_all[idx_three_of_a_kind - 1..]
                    .iter()
                    .position(|&x| {
                        x.0 == rank1 as i64 && x.3 == rank2 as i64 && x.4 == rank3 as i64
                    })
                    .unwrap()
        }
        Category::TwoPair(rank1, rank2, rank3) => {
            idx_two_pair
                + hands_all[idx_two_pair - 1..]
                    .iter()
                    .position(|&x| {
                        x.0 == rank1 as i64 && x.2 == rank2 as i64 && x.4 == rank3 as i64
                    })
                    .unwrap()
        }
        Category::OnePair(rank1, rank2, rank3, rank4) => {
            idx_one_pair
                + hands_all[idx_one_pair - 1..]
                    .iter()
                    .position(|&x| {
                        x.0 == rank1 as i64
                            && x.2 == rank2 as i64
                            && x.3 == rank3 as i64
                            && x.4 == rank4 as i64
                    })
                    .unwrap()
        }
        Category::HighCard(hand) => {
            let rank1 = hand.cards[0].rank;
            let rank2 = hand.cards[1].rank;
            let rank3 = hand.cards[2].rank;
            let rank4 = hand.cards[3].rank;
            let rank5 = hand.cards[4].rank;

            idx_high_card
                + hands_all[idx_high_card - 1..]
                    .iter()
                    .position(|&x| {
                        x.0 == rank1 as i64
                            && x.1 == rank2 as i64
                            && x.2 == rank3 as i64
                            && x.3 == rank4 as i64
                            && x.4 == rank5 as i64
                    })
                    .unwrap()
        }
    };

    writeln!(out, "{}", 7462 - ret + 1).unwrap();
}
