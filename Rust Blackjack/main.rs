use rand::{self, random_range};
use std::{io::{Write, stdin, stdout}};
/*
    Card is a struct that contains the card
    and just does some kinda cool things :3
*/
struct Card{
    suit:   u8,
    value:  u8,
}

fn print_card(card: &Card){
    let mut suit: String = String::new();
    let value: String;

    match card.value{
        1 =>  value = "Ace".to_owned(),
        11 => value = "Jack".to_owned(),
        12 => value = "Queen".to_owned(),
        13 => value = "King".to_owned(),
        _ =>  value = card.value.to_string(),
    }
    match card.suit{
        1 => suit = "Hearts".to_owned(),
        2 => suit = "Spades".to_owned(),
        3 => suit = "Clubs".to_owned(),
        4 => suit = "Diamonds".to_string(),
        _ => println!("Okay."),
    }


    print!("{} of {}", value, suit);
}

fn deal_card(src: &mut Vec<Card>,  dest: &mut Vec<Card>)-> u8{
    let rand = random_range(0..src.len());
    let dealt_card: Card = src.remove( rand);

    let mut dealt_val = dealt_card.value;
    if dealt_val > 10 {
        dealt_val = 10;
    }

    dest.push(dealt_card);
    return dealt_val;
}

fn deal_user(src: &mut Vec<Card>, user: &mut Vec<Card>, val_count: u8)-> u8{
    let val: u8 = deal_card(src, user);
    let res:u8;

    if val == 1 {
        // if we're below or equal to 10.
        if val_count <= 10 {
            // we're fine, just 
            // add 11.
            res = val_count + 11;
        }else{
            // we're too high just add 1
            res = val_count + val;
        }
    }else{
        // normal.
        res = val_count + val;
    }

    return res;
}
fn main(){
    let _=stdout().flush();

    let mut deck: Vec<Card> = Vec::new();   // the big deck wawa
    let mut deal: Vec<Card> = Vec::new();   // the dealer's deck
    let mut user: Vec<Card> = Vec::new();   // our deck for stuff.

    let mut user_val: u8 = 0;
    let mut dealer_val: u8 = 0;

    // initialize the deck 
    for x in 1..13+1 {
        // and to handle many suits
        for y in 1..4+1 {
            deck.push(
                Card{
                    suit:  y,
                    value: x,
                }
            );
            // 
        }
    }

    println!("Welcome to blackjack! You will be dealt 2 cards");

    user_val =   deal_user(  &mut deck, &mut user, user_val);
    user_val =   deal_user(  &mut deck, &mut user, user_val);
    dealer_val = deal_user(&mut deck, &mut deal, dealer_val);
    dealer_val = deal_user(&mut deck, &mut deal, dealer_val);

    let mut x: String = String::new();

    // now the game will start.
    while x != "stand"{
        // three displays
        // your value, cards, 
        // and 1st card of dealer
        println!("Your value: {}", user_val);
        for i in 0..user.len() {
            print!("User Card: ");
            print_card(&user[i]);
            println!("");
        }
        print!("First Dealer Card: ");
        print_card(&deal[0]);
        println!("");


        // prompt user
        println!("Will you hit or stand: ");
        let mut input_line = String::new();
        stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        x = input_line.trim().to_lowercase();

        // if we're hitting
        if x == "hit"{
            // ye
            user_val = deal_user(&mut deck, &mut user, user_val);
            print!("You drew: ");
            print_card(&user[user.len()-1]);
            println!("!");
    
            if user_val > 21 {
                println!("You've gone bust!\nFinal value: {}\nGame over.", user_val);
                return;
            }
        }
    }

    // now that we're done hitting, let's increment the dealer value!
    while dealer_val < user_val {
        println!("Current dealer value {}", dealer_val);
        dealer_val = deal_user(&mut deck, &mut deal, dealer_val);
    }


    // if the dealer went over 21 or its lower than you still, you win.
    if dealer_val > 21 || user_val > dealer_val {
        if dealer_val > 21 {
            println!("Dealer went bust!");
        }
        println!("You win!");
    }else{
        println!("you lose... :(");
    }
    for i in 0..deal.len() {

        print!("Dealer Card: ");
        print_card(&deal[i]);
        println!("");
}
    println!("Your Value: {} Dealer Value: {}", user_val, dealer_val);
}
