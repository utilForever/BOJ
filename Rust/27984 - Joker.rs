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
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
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
    Triple(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Hand),
    FullHouse(Rank, Rank),
    Quadruple(Rank, Rank),
    StraightFlush(Rank),
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
            return Category::StraightFlush(self.cards[0].rank);
        }

        // Check quadruple
        if ranks[0].1 == 4 {
            return Category::Quadruple(ranks[0].0, ranks[1].0);
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
            return Category::Straight(ranks[0].0);
        }

        // Check triple
        if ranks[0].1 == 3 {
            return Category::Triple(ranks[0].0, ranks[1].0, ranks[2].0);
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
                Category::Triple(_, _, _) => 4,
                Category::Straight(_) => 5,
                Category::Flush(_) => 6,
                Category::FullHouse(_, _) => 7,
                Category::Quadruple(_, _) => 8,
                Category::StraightFlush(_) => 9,
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
                        Category::Triple(rank1, rank2, rank3),
                        Category::Triple(other_rank1, other_rank2, other_rank3),
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
                        Category::Quadruple(rank1, rank2),
                        Category::Quadruple(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (Category::StraightFlush(rank), Category::StraightFlush(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    _ => None,
                };

                ret.filter(|compare| matches!(compare, Ordering::Less | Ordering::Greater))
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let convert_card = |raw_suit: i64, raw_rank: i64| -> Card {
        let suit = match raw_suit {
            0 => Suit::Club,
            1 => Suit::Diamond,
            2 => Suit::Heart,
            3 => Suit::Spade,
            _ => panic!("Invalid suit"),
        };
        let rank = match raw_rank {
            1 => Rank::Ace,
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
            _ => panic!("Invalid rank"),
        };

        Card { rank, suit }
    };

    let mut cards_raw = vec![(0, 0); 4];
    let mut cards = Vec::new();

    for i in 0..4 {
        cards_raw[i] = (scan.token::<i64>(), scan.token::<i64>());
        cards.push(convert_card(cards_raw[i].0, cards_raw[i].1));
    }

    let mut ret: Option<Hand> = None;
    let (mut suit, mut rank) = (0, 0);

    for i in 0..=3 {
        for j in 1..=13 {
            if cards_raw.iter().any(|&card| card == (i, j)) {
                continue;
            }

            let hand = Hand::new(
                [
                    cards[0].clone(),
                    cards[1].clone(),
                    cards[2].clone(),
                    cards[3].clone(),
                    convert_card(i, j),
                ]
                .to_vec(),
            );

            if ret.is_none() || ret.as_ref().unwrap().partial_cmp(&hand) == Some(Ordering::Less) {
                ret = Some(hand);
                suit = i;
                rank = j;
            }
        }
    }

    writeln!(out, "{suit} {rank}").unwrap();
}
