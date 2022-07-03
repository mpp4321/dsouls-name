use colored::{Color, Colorize};
use util::pick_from;
use util::words::*;
use std::fs::File;
use std::io::{self, BufRead};
use string_builder::Builder;

mod util;

#[allow(dead_code)]
pub enum PreBuiltChoice {
    StaticChoice(&'static str),
    AlwaysChoose,
    ChanceChoose(f64),
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
fn check_choice(
    pbc: &PreBuiltChoice,
    working_string: &mut Builder,
    strings: &[String],
    choice_fn: fn(&mut Builder, &[String]),
) {
    match pbc {
        PreBuiltChoice::AlwaysChoose => choice_fn(working_string, strings),
        PreBuiltChoice::ChanceChoose(chance) => {
            if rand::random::<f64>() < *chance {
                choice_fn(working_string, strings);
            }
        }
        PreBuiltChoice::StaticChoice(a) => {
            working_string.append(a.as_bytes());
        }
    }
}

const CHOOSE_NOUN: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;
const CHOOSE_PREFIX: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;
const CHOOSE_TITLE: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;
const CHOOSE_SUFFIX: PreBuiltChoice = PreBuiltChoice::AlwaysChoose;

pub fn generate_random_title() -> String {
    let mut working_name: Builder = Builder::default();

    let nouns_ref: &[String] = &*NOUNS;
    let adjectives_ref: &[String] = &*ADJECTIVES;
    let places_ref: &[String] = &*PLACES;
    let suffixes_ref: &[String] = &*SUFFIXES;
    let titles_ref: &[String] = &*TITLES;

    // Append adjective
    check_choice(
        &CHOOSE_PREFIX,
        &mut working_name,
        adjectives_ref,
        |build, strings| {
            let a = pick_from(strings);
            build.append(a.as_bytes());
            build.append(" ".as_bytes());
        },
    );

    // Append noun
    check_choice(
        &CHOOSE_NOUN,
        &mut working_name,
        nouns_ref,
        |build, strings| {
            let a = pick_from(strings).clone();
            build.append(a.as_bytes());
            // NO space because of the comma
            build.append("".as_bytes());
        },
    );

    // Append either
    // 0 => " of the [noun]"
    // 1 => " of [places]"
    // 2 => " of [suffix]"
    // 3 => " of [noun]"
    match rand::random::<u8>() % 4 {
        0 => {
            do_title(titles_ref, &mut working_name);
            check_choice(
                &CHOOSE_SUFFIX,
                &mut working_name,
                nouns_ref,
                |build, strings| {
                    let a = pick_from(strings).clone();
                    build.append(" of the ".as_bytes());
                    build.append(a.as_bytes());
                },
            );
        }
        1 => {
            do_title(titles_ref, &mut working_name);
            check_choice(
                &CHOOSE_SUFFIX,
                &mut working_name,
                places_ref,
                |build, strings| {
                    let a = pick_from(strings).clone();
                    build.append(" of ".as_bytes());
                    build.append(a.as_bytes());
                },
            );
        }
        2 => {
            do_title(titles_ref, &mut working_name);
            check_choice(
                &CHOOSE_SUFFIX,
                &mut working_name,
                suffixes_ref,
                |build, strings| {
                    let a = pick_from(strings).clone();
                    build.append(" of ".as_bytes());
                    build.append(a.as_bytes());
                },
            );
        }
        3 => {
            working_name.append(", ".as_bytes());
            check_choice(
                &CHOOSE_SUFFIX,
                &mut working_name,
                nouns_ref,
                |build, strings| {
                    let a = pick_from(strings).clone();
                    // Space because no comma
                    build.append("\"The ".as_bytes());
                    build.append(a.as_bytes());
                    build.append("\"".as_bytes());
                },
            );
        }
        _ => {}
    }
    return working_name.string().expect("Failed to build string");
}

#[allow(dead_code)]
pub mod name_code {
    use super::*;

    // Legacy code
    fn do_name_with_sentence() {
        let file_with_names = File::open("res/names.txt").expect("Did not find names.txt");
        let lines = io::BufReader::new(file_with_names)
            .lines()
            .collect::<Vec<Result<String, _>>>();

        for _ in 0..10 {
            println!("{}, {}", get_random_name(&lines), generate_random_title());
        }
    }

    fn get_random_name(lines: &Vec<Result<String, io::Error>>) -> String {
        let n = rand::random::<usize>() % lines.len();
        lines[n]
            .as_ref()
            .map(Clone::clone)
            .expect("Error picking line")
    }
}

fn generate_true_color() -> Color {
    Color::TrueColor {
        r: rand::random::<u8>(),
        g: rand::random::<u8>(),
        b: rand::random::<u8>(),
    }
}

// This is being used as an optional binary
#[allow(dead_code)]
fn main() {
    let names = std::env::args()
        .nth(1)
        .unwrap_or("10".to_string())
        .parse::<i32>()
        .unwrap();

    for _ in 0..names {
        println!(
            "{} | {}",
            generate_random_title(),
            "#".color(generate_true_color())
        );
    }
}
