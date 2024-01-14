// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
  Backstage(i32, String),
  Vip(i32, String),
  Standard(i32)
}

fn main() {
  let tickets = vec![
    Ticket::Backstage(32, "Lewis".to_owned()),
    Ticket::Vip(42, "Max".to_owned()),
    Ticket::Standard(23),
  ];

  for ticket in tickets {
    match ticket {
      Ticket::Backstage(price, holder) => {
        println!("Backstage Ticket Holder: {:?}, Price: {:?}", holder, price)
      },
      Ticket::Vip(price, holder) => {
        println!("VIP ticket Holder: {:?}, Price: {:?}", holder, price)
      },
      Ticket::Standard(price) => println!("Standard ticket. Price: {:?}", price)
    }
  }
}
