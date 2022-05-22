mod request_handler;

use lazy_static::lazy_static;
use string_builder::Builder;
use std::fs::File;
use std::io::{BufRead, self};

#[allow(dead_code)]
enum PreBuiltChoice {
    StaticChoice(&'static str),
    AlwaysChoose,
    ChanceChoose(f64),
}

lazy_static! {
    static ref NOUNS: Vec<String> = read_lines_vec("res/nouns.txt");
    static ref ADJECTIVES: Vec<String> = read_lines_vec("res/adjectives.txt");
    static ref PLACES: Vec<String> = read_lines_vec("res/places.txt");
    static ref SUFFIXES: Vec<String> = read_lines_vec("res/suffixes.txt");
    static ref TITLES: Vec<String> = read_lines_vec("res/titles.txt");
}

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

/// tries to append ", [title]" to the working_name
fn do_title(titles_ref: &[String], working_name: &mut Builder) {
    check_choice(&CHOOSE_TITLE, working_name, titles_ref, |build, strings| {
        let a = pick_from(strings).clone();
        build.append(", ".as_bytes());
        build.append(a.as_bytes());
    });
}

/// Runs the choice_fn closure on the working_string
/// if the PreBuiltChoice passes
fn check_choice(pbc: &PreBuiltChoice,
                working_string: &mut Builder,
                strings: &[String],
                choice_fn: fn(&mut Builder, &[String])) {
    match pbc {
        PreBuiltChoice::AlwaysChoose => choice_fn(working_string, strings),
        PreBuiltChoice::ChanceChoose(chance) => { 
            if rand::random::<f64>() < *chance {
                choice_fn(working_string, strings);
            }
        },
        PreBuiltChoice::StaticChoice(a) => {
            working_string.append(a.as_bytes());
        },
    }
}

const CHOOSE_NOUN: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;
const CHOOSE_PREFIX: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;
const CHOOSE_TITLE: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;
const CHOOSE_SUFFIX: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;

fn generate_random_title() -> String {
    let mut working_name: Builder = Builder::default();
    
    let nouns_ref: &[String] = &*NOUNS;
    let adjectives_ref: &[String] = &*ADJECTIVES;
    let places_ref: &[String] = &*PLACES;
    let suffixes_ref: &[String] = &*SUFFIXES;
    let titles_ref: &[String] = &*TITLES;

    // Append adjective
    check_choice(&CHOOSE_PREFIX, &mut working_name, adjectives_ref, |build, strings| {
        let a = pick_from(strings); 
        build.append(a.as_bytes());
        build.append(" ".as_bytes());
    });

    // Append noun
    check_choice(&CHOOSE_NOUN, &mut working_name, nouns_ref, |build, strings| {
        let a = pick_from(strings).clone();
        build.append(a.as_bytes());
        build.append(" ".as_bytes());
    });

    // Append either 
    // 0 => " of the [noun]"
    // 1 => " of [places]"
    // 2 => " of [suffix]"
    // 3 => " of [noun]"
    match rand::random::<u8>() % 4 {
        0 => {
            do_title(titles_ref, &mut working_name);
            check_choice(&CHOOSE_SUFFIX, &mut working_name, nouns_ref, |build, strings| {
                let a = pick_from(strings).clone();
                build.append(" of the ".as_bytes());
                build.append(a.as_bytes());
            });
        },
        1 => {
            do_title(titles_ref, &mut working_name);
            check_choice(&CHOOSE_SUFFIX, &mut working_name, places_ref, |build, strings| {
                let a = pick_from(strings).clone();
                build.append(" of ".as_bytes());
                build.append(a.as_bytes());
            });
        },
        2 => {
            do_title(titles_ref, &mut working_name);
            check_choice(&CHOOSE_SUFFIX, &mut working_name, suffixes_ref, |build, strings| {
                let a = pick_from(strings).clone();
                build.append(" of ".as_bytes());
                build.append(a.as_bytes());
            });
        },
        3 => {
            check_choice(&CHOOSE_SUFFIX, &mut working_name, nouns_ref, |build, strings| {
                let a = pick_from(strings).clone();
                build.append("of ".as_bytes());
                build.append(a.as_bytes());
            });
        },
        _ => {},
    }
    return working_name.string().expect("Failed to build string");
}

#[allow(dead_code)]
mod name_code {
    use super::*;

    // Legacy code
    fn do_name_with_sentence() {
        let file_with_names = File::open("res/names.txt").expect("Did not find names.txt");
        let lines = io::BufReader::new(file_with_names).lines().collect::<Vec<Result<String, _>>>();

        for _ in 0..10 {
            println!("{}, {}", get_random_name(&lines), generate_random_title());
        }
    }

    fn get_random_name(lines: &Vec<Result<String, io::Error>>) -> String {
        let n = rand::random::<usize>() % lines.len();
        lines[n].as_ref().map(Clone::clone).expect("Error picking line")
    }
}

fn main() {
    for _ in 0..10 {
        println!("{}", generate_random_title());
    }
}
