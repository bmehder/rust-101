// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
  age: i32,
  name: String,
  colour: String
}

impl Person {
  fn print(&self) {
    println!("Name: {:?} Favourite Colour: {:?}", self.name, self.colour);
  }
}

fn main() {
  let people = vec![
    Person {
      age: 9,
      name: "Lewis".to_owned(),
      colour: "Silver".to_owned()
    },
    Person {
      age: 7,
      name: "Max".to_owned(),
      colour: "Blue".to_owned()
    },
    Person {
      age: 29,
      name: "Lando".to_owned(),
      colour: "Orange".to_owned()
    },
  ];

  for person in people {
    if person.age <= 10 {
      person.print();
    }
  }
}
