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

impl Rank {
    fn from_i32(value: i32) -> Rank {
        match value {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl Suit {
    fn from_i32(value: i32) -> Suit {
        match value {
            0 => Suit::Diamond,
            1 => Suit::Club,
            2 => Suit::Heart,
            3 => Suit::Spade,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    #[inline]
    fn get_category(cards: &[Card]) -> Category {
        let is_suit_all_same = cards.iter().all(|card| card.suit == cards[0].suit);
        let is_straight_normal = cards
            .windows(2)
            .all(|window| window[0].rank as i64 == window[1].rank as i64 + 1);
        let is_straight_baby = cards[0].rank == Rank::Ace
            && cards[1].rank == Rank::Five
            && cards[2].rank == Rank::Four
            && cards[3].rank == Rank::Three
            && cards[4].rank == Rank::Two;
        let is_straight = is_straight_normal || is_straight_baby;

        let ranks: HashMap<Rank, i64> = cards.iter().fold(HashMap::new(), |mut acc, card| {
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
            if cards[4].rank == Rank::Ten {
                return Category::RoyalFlush;
            } else if is_straight_baby {
                return Category::StraightFlush(Rank::Five);
            } else {
                return Category::StraightFlush(cards[0].rank);
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
            return Category::Flush(Hand::new(cards.to_vec()));
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
        Category::HighCard(Hand::new(cards.to_vec()))
    }
}

const PERMUTATIONS: [[usize; 5]; 21] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 6],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 4, 6],
    [0, 1, 2, 5, 6],
    [0, 1, 3, 4, 5],
    [0, 1, 3, 4, 6],
    [0, 1, 3, 5, 6],
    [0, 1, 4, 5, 6],
    [0, 2, 3, 4, 5],
    [0, 2, 3, 4, 6],
    [0, 2, 3, 5, 6],
    [0, 2, 4, 5, 6],
    [0, 3, 4, 5, 6],
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 6],
    [1, 2, 3, 5, 6],
    [1, 2, 4, 5, 6],
    [1, 3, 4, 5, 6],
    [2, 3, 4, 5, 6],
];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let cards_all = (0..13)
        .flat_map(|rank| {
            (0..4).map(move |suit| Card {
                rank: Rank::from_i32(rank),
                suit: Suit::from_i32(suit),
            })
        })
        .collect::<Vec<_>>();

    let convert_card = |raw_card: &str| -> Card {
        let suit = match &raw_card[0..1] {
            "D" => Suit::Diamond,
            "C" => Suit::Club,
            "H" => Suit::Heart,
            "S" => Suit::Spade,
            _ => panic!("Invalid suit"),
        };
        let rank = match &raw_card[1..2] {
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

    loop {
        let mut cards_alice = vec![Card::new(Rank::Two, Suit::Club); 7];
        let mut cards_bob = vec![Card::new(Rank::Two, Suit::Club); 7];

        let card = scan.token::<String>();

        if card == "#" {
            break;
        }

        cards_alice[0] = convert_card(&card);
        cards_alice[1] = convert_card(&scan.token::<String>());

        for card in cards_bob.iter_mut().take(2) {
            *card = convert_card(&scan.token::<String>());
        }

        for i in 0..3 {
            let card = convert_card(&scan.token::<String>());
            cards_alice[i + 2] = card;
            cards_bob[i + 2] = card;
        }

        let mut cards_except_duplicated = cards_all.clone();

        for card in cards_alice[0..5].iter() {
            cards_except_duplicated.retain(|&x| x != *card);
        }

        for card in cards_bob[0..2].iter() {
            cards_except_duplicated.retain(|&x| x != *card);
        }

        let mut cnt_win_alice = 0;
        let mut cnt_win_bob = 0;
        let mut cnt_draw = 0;

        for i in 0..cards_except_duplicated.len() - 1 {
            cards_alice[5] = cards_except_duplicated[i];
            cards_bob[5] = cards_except_duplicated[i];

            for j in i + 1..cards_except_duplicated.len() {
                cards_alice[6] = cards_except_duplicated[j];
                cards_bob[6] = cards_except_duplicated[j];

                let mut alice_category_max = Category::HighCard(Hand::new(cards_alice.to_vec()));
                let mut bob_category_max = Category::HighCard(Hand::new(cards_bob.to_vec()));

                let mut cards_alice = cards_alice.to_vec();
                let mut cards_bob = cards_bob.to_vec();

                cards_alice.sort_by(|a, b| b.rank.cmp(&a.rank));
                cards_bob.sort_by(|a, b| b.rank.cmp(&a.rank));

                for permutation in PERMUTATIONS.iter() {
                    alice_category_max = alice_category_max.max(Hand::get_category(
                        [
                            cards_alice[permutation[0]],
                            cards_alice[permutation[1]],
                            cards_alice[permutation[2]],
                            cards_alice[permutation[3]],
                            cards_alice[permutation[4]],
                        ]
                        .as_ref(),
                    ));
                    bob_category_max = bob_category_max.max(Hand::get_category(
                        [
                            cards_bob[permutation[0]],
                            cards_bob[permutation[1]],
                            cards_bob[permutation[2]],
                            cards_bob[permutation[3]],
                            cards_bob[permutation[4]],
                        ]
                        .as_ref(),
                    ));
                }

                match alice_category_max.cmp(&bob_category_max) {
                    Ordering::Less => cnt_win_bob += 1,
                    Ordering::Equal => cnt_draw += 1,
                    Ordering::Greater => cnt_win_alice += 1,
                }
            }
        }

        writeln!(
            out,
            "{:.6}",
            cnt_win_alice as f64 / (cnt_win_alice + cnt_win_bob + cnt_draw) as f64
        )
        .unwrap();
    }
}
