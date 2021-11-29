use ansi_term::Colour;

fn main() {
    println!("This is {} in  colour, {} in colour and {} in colour",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}
