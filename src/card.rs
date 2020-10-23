pub mod card {
    //0.17.1
    use strum_macros::EnumIter; //0.17.1

    #[derive(Debug, EnumIter, Copy, Clone)]
    pub enum Suit {
        Spades,
        Clubs,
        Hearts,
        Diamonds,
    }

    #[derive(Debug, EnumIter, Copy, Clone)]
    pub enum Value {
        Ace,
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
    }

    #[derive(Debug)]
    pub struct Card {
        pub suit: Suit,
        pub value: Value,
    }

    impl Card {
        pub fn to_string(&self) -> String {
            format!("{:?} of {:?}.", self.value, self.suit)
        }
    }
}

pub mod deck {
    use crate::card::card::Card;
    use crate::card::card::Suit;
    use crate::card::card::Value;

    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use strum::IntoEnumIterator;

    #[derive(Debug)]
    pub struct Deck {
        deck: Vec<Card>,
    }
    pub fn new() -> Deck {
        let mut deck = Vec::new();
        for value in Value::iter() {
            for suit in Suit::iter() {
                deck.push(Card {
                    suit: suit,
                    value: value,
                });
            }
        }
        Deck { deck: deck }
    }

    impl Deck {
        pub fn shuffle(&mut self) -> &Deck {
            let mut rng = thread_rng();
            self.deck.shuffle(&mut rng);
            self
        }

        pub fn pretty_print(self) {
            for card in self.deck {
                println!("{:?} of {:?}.", card.value, card.suit);
            }
        }
    }
}
