#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let n = rand::random::<u8>() % 4 + 1;
        match n{
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            4 => Suit::Spade,
            _ => unreachable!(),
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            4 => Suit::Spade,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let n = rand::random::<u8>() % 13 + 1;
        match n {
            1 => Rank::Ace,
            2..=10 => Rank::Number(n),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match card {
        Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        } => true,
        _ => false,
    }
}
