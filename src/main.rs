mod request_handler;

use lazy_static::lazy_static;
use string_builder::Builder;
use std::fs::File;
use std::io::{BufRead, self};

lazy_static! {
    static ref NOUNS: Vec<String> = read_lines_vec("res/nouns.txt");
    static ref ADJECTIVES: Vec<String> = read_lines_vec("res/adjectives.txt");
    static ref PLACES: Vec<String> = read_lines_vec("res/places.txt");
    static ref SUFFIXES: Vec<String> = read_lines_vec("res/suffixes.txt");
    static ref TITLES: Vec<String> = read_lines_vec("res/titles.txt");
}

const DECKED: bool = true;

fn read_lines_vec(name: &'static str) -> Vec<String> {
    let file_with_names = File::open(name).expect("Did not find file");
    let lines = io::BufReader::new(file_with_names).lines().collect::<Vec<Result<String, _>>>();
    let lines: Vec<String> = lines.into_iter().flat_map(|a| a).collect();
    return lines;
}

fn pick_from<T>(items: &[T]) -> &T {
    let len = items.len();
    let random_len = rand::random::<usize>() % len;
    return &items[random_len];
}

fn do_title(choose_title: bool, titles_ref: &[String], working_name: &mut Builder) {
    if choose_title {
        let a = pick_from(titles_ref).clone();
        working_name.append(", ");
        working_name.append(a.as_bytes());
    }
}

fn generate_random_name() -> String {
    let mut working_name: Builder = Builder::default();

    // An adjective prefix
    let choose_prefix = rand::random::<bool>() || DECKED;


    // A word from the suffix list
    let choose_suffix = rand::random::<bool>() || DECKED;

    // Title
    let choose_title = (rand::random::<bool>() || DECKED) && choose_suffix;
    
    let nouns_ref: &[String] = &*NOUNS;
    let adjectives_ref: &[String] = &*ADJECTIVES;
    let places_ref: &[String] = &*PLACES;
    let suffixes_ref: &[String] = &*SUFFIXES;
    let titles_ref: &[String] = &*TITLES;

    let noun = pick_from(nouns_ref);
    if choose_prefix {
        let a = pick_from(adjectives_ref); 
        working_name.append(a.as_bytes());
        working_name.append(" ".as_bytes());
    }
    working_name.append(noun.as_bytes());
    if choose_suffix {
        match rand::random::<i8>() % 4 {
            0 => {
                do_title(choose_title, titles_ref, &mut working_name);
                let a = pick_from(nouns_ref).clone();
                working_name.append(" of the ".as_bytes());
                working_name.append(a.as_bytes());
            },
            1 => {
                do_title(choose_title, titles_ref, &mut working_name);
                let a = pick_from(places_ref).clone();
                working_name.append(" of ".as_bytes());
                working_name.append(a.as_bytes());
            },
            2 => {
                do_title(choose_title, titles_ref, &mut working_name);
                let a = pick_from(suffixes_ref).clone();
                working_name.append(" of ".as_bytes());
                working_name.append(a.as_bytes());
            },
            3 => {
                let a = pick_from(nouns_ref).clone();
                working_name.append(" ".as_bytes());
                working_name.append(a.as_bytes());
            },
            _ => {},
        }
    }
    return working_name.string().expect("Failed to build string");
}

fn do_name_with_sentence(lines: &Vec<Result<String, io::Error>>) {
    let n = rand::random::<usize>() % lines.len();
    let nth = lines[n].as_ref().map(Clone::clone).expect("Picked an error line.");

    println!("{}, {}", nth, generate_random_name());
}

fn main() {
    let file_with_names = File::open("res/names.txt").expect("Did not find names.txt");
    let lines = io::BufReader::new(file_with_names).lines().collect::<Vec<Result<String, _>>>();
    for _ in 0..10 {
        do_name_with_sentence(&lines);
    }
}
