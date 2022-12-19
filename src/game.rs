use super::*;
use card::*;
use std::fmt;

/// right here we can see the operation where the program will
/// be dealing the first three crads two to the player
/// and one to the dealer like your standard black jack game
pub fn deal_players(deck: &mut Deck, dealer: &mut dyn Person, player: &mut dyn Person) {
    player.deal_card(deck.get_card());
    dealer.deal_card(deck.get_card());
    player.deal_card(deck.get_card());
}
/// Here we can see the showing of the card results hint the name get_score
/// in this operation we can see where thr program will assert
/// the values to the 4 face cards giving 10 to the Knight, Queen, and King, and 1 to the Ace
/// 
pub fn get_score(player: & dyn Person) -> Score {
    let hand = &player.get_hand();
    let mut result: u8 = 0;
    for card in hand.iter() {
        match card.rank {
            Rank::Knight => result += 10,
            Rank::Queen => result += 10,
            Rank::King => result += 10,
            Rank::Ace => result += 1,
            x => result += x as u8,
        }
    }
    
    /// here we can see the code that shows that if the hand that either you or
    /// the dealer has is higher then 10 when it ranks your score
    /// if you get an ace it is plus 10 instead of one
    for card in hand.iter() {
        if let Rank::Ace = card.rank {
            if result + 10 <= 21 {
                result += 10;
            }
        }
    }
    /// This part shows if you get above 21 you have busted
    if result > 21 {
        return Score::Busted;
    }
    /// This part shows if you hit 21 with either 2 or more cards you strike black jack
    if result == 21 && hand.len() == 2 {
        return Score::Blackjack
    }
    Score::Points(result)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Score {
    Busted,
    Points(u8),
    Blackjack,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Score::Blackjack => write!(f, "{:?}", &self),
            Score::Busted => write!(f, "{:?}", &self),
            Score::Points(num) => write!(f, "{}", num),
        }
    }
}

pub fn get_winner<'a>(dealer: &'a dyn Person, player: &'a dyn Person) -> Option<&'a dyn Person> {
    let player_score = get_score(player); // Get their scores.
    let dealer_score = get_score(dealer);
    if player_score == dealer_score { // If they have the same score but not blackjack
        if dealer_score > Score::Points(16) && dealer_score != Score::Blackjack { 
            return Some(dealer); // The dealer wins from 17 and up
        } else {
            return None;
        }
    };
    if player_score > dealer_score { // Otherwise player with the highest score wins. 
        return Some(player);
    } else {
        return Some(dealer);
    }
}
/// Down here we have all the tests for the code where
/// the ace is involved 
/// if you look at the names of the functions 
/// they are trying to show that theyre are
/// many differnt options that the ace 
/// could work for such as just 7+ace is 18 because it splits the 7+11
/// then 7+ace+knight is also 18 because it breaks the ace to the 1 so it is 7+1+ 10 for the night
#[test]
fn queen_and_ace_is_blackjack() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Clovers, Rank::Queen));
    assert_eq!(Score::Blackjack, get_score(&mut player));
}
#[test]
fn seven_and_ace_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    assert_eq!(Score::Points(18), get_score(&mut player));
}
#[test]
fn seven_ace_and_knight_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clovers, Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Tiles, Rank::Knight));
    assert_eq!(Score::Points(18), get_score(&mut player));
}
#[test]
fn five_aces_is15() {
    let mut player = Player::new("Test");
    for _num in 0..5 {
        player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    }
    assert_eq!(Score::Points(15), get_score(&mut player));
}

#[test]
fn blackjack_wins_over_21() {
    let mut player = Player::new("Test");
    let mut dealer = Player::new("Test2");
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));

    dealer.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));
    dealer.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));
    dealer.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    let winner = game::get_winner(&dealer, &player).unwrap();
    assert_eq!(player.get_name(), winner.get_name());
}

#[test]
fn aces_shrink_when_needed() {
    let mut player = Player::new("Player");
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));
    player.deal_card(Card::new(card::Suit::Clovers, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Tiles, Rank::Ace));
    let score = game::get_score(&player);
    assert_eq!(Score::Points(12), score);

}