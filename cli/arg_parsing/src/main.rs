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
        .get_matches();
}
