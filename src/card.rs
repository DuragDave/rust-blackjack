use std::fmt;
use strum_macros::EnumIter;

#[derive(Clone)]
/// Here we are forming new struct for Card and making two new public variables for both Suit and Rank
/// This will allow for us to see that Suit and Rank are assigned to the Crad Struct
/// Then we can assign values and constarints to these variables
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
/// Here is where we are creating the "Card" by assigning it to both a suite and rank
impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
        }
    }
}
/// Right here we are creating the display and assigning it a format 
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", &self.rank, &self.suit)
    }
}

#[derive(EnumIter, Debug, PartialEq, Clone)]
#[allow(unused_variables, dead_code)]
/// Here we can see that we will be implementing what they types of Suits are that can be assigned to the Cards
/// They are differently named here then your average deck of cards which would be
/// Hearts, Diamonds, Clubs, and Spades
pub enum Suit {
    Hearts, 
    Tiles, 
    Clovers, 
    Pikes
}
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
#[allow(unused_variables, dead_code)]
/// In this part of the code we can see we are implementing the values that will go with the Ranks.
/// This will assign one of these values along with the suits above to the cards
/// to give us a whole card such as King of Tiles
pub enum Rank { // Todo: Refactor so Two = 2, etc and in game use 'Two as u8' instead of that match statement. 
    Ace, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, 
    Eight = 8, Nine = 9, Ten = 10, Knight, Queen, King
}

