use clap::{crate_version, App, Arg, SubCommand};
use rand::Rng;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

fn main() {
    let matches = App::new("speller")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("add")
                .about("add word to speller database")
                .arg(
                    Arg::with_name("word")
                        .help("word to add to database")
                        .required(true),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("add") {
        if matches.is_present("word") {
            add(matches.value_of("word").unwrap());
        }
    } else {
        run();
    }
}

fn get_database() -> String {
    if let Ok(mut val) = std::env::var("APPDATA") {
        val.push_str("\\speller.txt");
        val
    } else {
        String::from("speller.txt")
    }
}

fn add(word: &str) {
    let database_path = get_database();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(database_path)
        .unwrap();
    file.write_all(word.as_bytes()).unwrap();
    file.write_all(b"\r\n").unwrap();
}

fn run() {
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = Vec::new();
    for s in read_to_string(get_database()).unwrap().split("\r\n") {
        if !s.is_empty() {
            words.push(String::from(s));
        }
    }

    loop {
        let random_index = rng.gen_range(0, words.len());
        let word = String::from(&words[random_index]);
        for _ in 0..3 {
            print!("{} ", word);
            std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input);

            if &word != input.trim() {
                println!(" WRONG!");
            }
        }
    }
}
