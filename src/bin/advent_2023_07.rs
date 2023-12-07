const CARD_RANKS: &[u8] = b"23456789TJQKA";
type Card = u8;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    kind: HandKind,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: u32) -> Hand {
        // Unlike real poker, do not canonicalize the hand!
        //cards.sort_by(|a, b| a.cmp(b).reverse());
        // A strange, but low effort, method of determining the hand...
        let mut counts = [0; CARD_RANKS.len()];
        for card in cards.iter() {
            counts[*card as usize] += 1;
        }
        counts.sort_by(|a, b| a.cmp(b).reverse());
        let kind = if counts[0] == 5 {
            HandKind::FiveKind
        } else if counts[0] == 4 {
            HandKind::FourKind
        } else if counts[0] == 3 {
            if counts[1] == 2 {
                HandKind::FullHouse
            } else {
                HandKind::ThreeKind
            }
        } else if counts[0] == 2 {
            if counts[1] == 2 {
                HandKind::TwoPair
            } else {
                HandKind::OnePair
            }
        } else {
            HandKind::HighCard
        };
        Hand { cards, bid, kind }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            // If equal, we need to look at the cards.
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            // If not equal, the kind was enough to decide.
            x => x,
        }
    }
}

fn map_card(card: u8) -> Card {
    CARD_RANKS.iter().position(|x| *x == card).unwrap() as Card
}

fn main() {
    let mut hands: Vec<Hand> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (cards, bid) = line.split_once(' ').unwrap();
            assert_eq!(cards.len(), 5);
            let cards = cards.as_bytes();
            let cards = [
                map_card(cards[0]),
                map_card(cards[1]),
                map_card(cards[2]),
                map_card(cards[3]),
                map_card(cards[4]),
            ];
            let bid = bid.parse().unwrap();
            Hand::new(cards, bid)
        })
        .collect();
    hands.sort();
    println!(
        "Puzzle 1 answer: {}",
        hands
            .iter()
            .enumerate()
            .fold(0u64, |accumulator, (index, hand)| {
                accumulator + (index + 1) as u64 * hand.bid as u64
            })
    );
}
