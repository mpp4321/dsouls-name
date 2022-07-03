pub fn pick_from<T>(items: &[T]) -> &T {
    let len = items.len();
    let random_len = rand::random::<usize>() % len;
    return &items[random_len];
}

#[macro_export]
macro_rules! include_as_vec_string {
    ($path:expr) => {
        include_str!($path)
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };
}

pub mod words {
    lazy_static::lazy_static! {
        pub static ref NOUNS: Vec<String> = include_as_vec_string!("../res/nouns.txt");
        pub static ref ADJECTIVES: Vec<String> = include_as_vec_string!("../res/adjectives.txt");
        pub static ref PLACES: Vec<String> = include_as_vec_string!("../res/places.txt");
        pub static ref SUFFIXES: Vec<String> = include_as_vec_string!("../res/suffixes.txt");
        pub static ref TITLES: Vec<String> = include_as_vec_string!("../res/titles.txt");
    }
}
