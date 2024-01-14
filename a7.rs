// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
  Black,
  White,
  GrayThing
}

fn print_color_name(color: Color) {
  match color {
    Color::Black => println!("Black"),
    Color::White => println!("White"),
    Color::GrayThing => println!("Gray_Thing"),
  }
} 

fn main() {
  print_color_name(Color::GrayThing)
}
