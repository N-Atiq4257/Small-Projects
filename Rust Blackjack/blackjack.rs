use rand;
/*
    Card is a struct that contains the card
    and just does some kinda cool things :3
*/
struct Card{
    suit:   u8,
    value:  u8,
}

fn main(){
    let me = rand::rng();

    let mut deck: Vec<Card> = Vec::new();   // the big deck wawa
    let mut deal: Vec<Card> = Vec::new();   // the dealer's deck
    let mut user: Vec<Card> = Vec::new();   // our deck for stuff.

    // initialize the deck wawa
    for x in 1..13+1 {
        // and to handle many suits
        for y in 1..4+1 {
            deck.push(
                Card{
                    suit:  y,
                    value: x,
                }
            );
            // wawa
        }
    }

    


    for i in 0..deck.len() {
        println!("This card's value is {} and its value is {}", deck[i].suit, deck[i].value);
    }
}
