mod poker_eval {

    #[derive(PartialEq, Debug)]
    pub enum Suit {
        Spade = 0,
        Heart = 1,
        Club = 2,
        Diamond = 3,
    }

    // const SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];

    #[derive(PartialEq, Debug)]
    pub enum Value {
        Two = 0,
        Three = 1,
        Four = 2,
        Five = 3,
        Six = 4,
        Seven = 5,
        Eight = 6,
        Nine = 7,
        Ten = 8,
        Jack = 9,
        Queen = 10,
        King = 11,
        Ace = 12,
    }

    #[derive(PartialEq, Debug)]
    pub struct Card {
        suit: Suit,
        rank: Value,
    }

    impl Card {
        pub fn new(s: Suit, r: Value) -> Card {
            Card { suit: s, rank: r }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_constructor() {
        let c = Card::new(Suit::Spade, Value::Ace);
        assert_eq!(Suit::Spade, c.suit);
        assert_eq!(Value::Ace, c.rank);
    }
}
