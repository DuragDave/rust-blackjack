/// Here we are calling for the usage of the other crates so that the deck can be created 
/// This will take the values given to rank, and suit that make up card and create a deck with no repeating card 
use crate::Card;
use crate::Rank;
use crate::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

///Here we are implemnting a new struct which is Deck which will hold the values of card, suit, and rank and make 
/// a deck with no repating cards and 52 options of different individual cards
pub struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }
    /// Here we are creating a new card by calling for it to have 
    /// a rank and a suit and to iterate in between the gven options of rank and suit
    /// so we dont just get like 2 of hearts, 2 of hearts, 4 of hearts and so on as the deck when we try to play
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit.clone(), rank.clone()))
            }
        }
        cards.push(Card::new(Suit::Clovers, Rank::Ace));
        Self { cards }
    }
    /// At this part is where the program will get one of the generated cards 
    pub fn get_card(&mut self) -> Card {
        let card = self.cards.pop();
        match card {
            None => panic!("Not enough cards in deck."),
            Some(card) => card,
        }
    }
    /// At this part of the code we see shuffle where
    /// once the game is over it will shuffle the deck or before it starts
    /// so when you want to play over and over agin you will always have every card as an option in the deck
    /// this keeps your from being able to partially count the cards
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        &self.cards.shuffle(&mut rng);
    }
}
