use rand::Rng;

//For Suit
#[derive(Debug, PartialEq, Eq)]
pub enum Suit{
    Heart,
    Diamond,
    Spade,
    Club,
}
impl Suit{
    pub fn random() -> Suit{
        let  rng = rand::thread_rng().gen_range(1, 5);
        Self::translate(rng)
    }
    pub fn translate(value: u8) -> Suit{
        match  value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => panic!("Invalid suit value: {}", value),
        }
    }
}

//For Rank
#[derive(Debug, PartialEq, Eq)]
pub enum Rank{
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}
impl Rank{
    pub fn random() -> Rank{
        let rng = rand::thread_rng().gen_range(1, 14);
        Self::translate(rng)
    }
    pub fn translate(value: u8) -> Rank{
        match value {
            1 => Self::Ace,
            2..=10 => Rank::Number(value),
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => panic!("Not exist")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
//For Card
pub struct Card{
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool{
    card.suit == Suit::Spade  && card.rank == Rank::Ace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
