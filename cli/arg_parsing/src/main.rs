use clap::{Arg, App};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Jeklah")
        .about("Learning Rust")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A cool file."))
        .arg(Arg::with_name("num")
            .short("n")
            .long("name")
            .takes_value(true)
            .help("Five less thsn your favourite number."))
        .get_matches()

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favourite number is."),
        Some(s) => {
            match.s.parse::<i32>() {
                Ok(n) => println!("Your favourite number must be {}.", n+5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
