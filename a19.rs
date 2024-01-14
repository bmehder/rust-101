// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
  let mut furniture = HashMap::new();

  furniture.insert("Chairs", 5);
  furniture.insert("Beds", 3);
  furniture.insert("Tables", 2);
  furniture.insert("Couches", 0);

  let mut total = 0;

  for (key, value) in furniture.iter() {
    match value {
      0 => println!("{:?} is out of stock", key),
      _ => println!("{:?}: {:?}", key, value),
    }
    total += value
  }

  println!("Total furniture: {:?}", total);
}
