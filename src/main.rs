use clap::Parser;
use clap::builder::NonEmptyStringValueParser;
use rand::{RngExt, seq::IndexedRandom};
use std::{
    fs::{self, File},
    io::{self, BufRead},
    num::NonZeroUsize,
    path::{Path, PathBuf},
    process,
};

// read word list
fn read_dict<P: AsRef<Path>>(
    dict_path: P,
    min_chars: usize,
    max_chars: usize,
) -> io::Result<Vec<String>> {
    let file = File::open(dict_path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| {
            let chars = line.chars().count();
            chars >= min_chars && chars <= max_chars
        })
        .collect();

    Ok(lines)
}

#[derive(Parser, Debug)]
#[command(name = "pwgen", version = "1.0", about = "Password generator")]
struct Cli {
    /// Path to the dictionary
    #[arg(
        short = 'f',
        long = "dict-file",
        value_name = "FILE",
        default_value = "words.txt"
    )]
    dict_file: PathBuf,

    /// Number of words per password (>0)
    #[arg(
        short = 'w',
        long = "word-count",
        value_name = "COUNT",
        default_value = "4"
    )]
    word_count: NonZeroUsize,

    /// Number of passwords to generate (>0)
    #[arg(
        short = 'c',
        long = "pw-count",
        value_name = "COUNT",
        default_value = "10"
    )]
    pw_count: NonZeroUsize,

    /// Minimum number of characters per word (>0)
    #[arg(
        short = 'm',
        long = "min-char-per-word",
        value_name = "COUNT",
        default_value = "4"
    )]
    min_chars: NonZeroUsize,

    /// Maximum number of characters per word (>0, >=min-char-per-word)
    #[arg(
        short = 'n',
        long = "max-char-per-word",
        value_name = "COUNT",
        default_value = "5"
    )]
    max_chars: NonZeroUsize,

    /// Separator alphabet between words
    #[arg(
        short = 's',
        long = "separator",
        value_name = "SEP",
        default_value = "!@$%^&*+=:|~",
        value_parser = NonEmptyStringValueParser::new(),
    )]
    separator: String,
}

fn main() {
    let args = Cli::parse();

    // validate args
    if args.max_chars < args.min_chars {
        eprintln!(
            "Error: Maximum number of characters per word ({}) cannot be less than minimum ({})",
            args.max_chars, args.min_chars
        );
        process::exit(1);
    }
    if !fs::metadata(&args.dict_file).is_ok() {
        eprintln!(
            "Error: Dictionary '{}' does not exist",
            &args.dict_file.display()
        );
        process::exit(1);
    }

    // parse dictionary
    let dict = match read_dict(
        &args.dict_file,
        args.min_chars.into(),
        args.max_chars.into(),
    ) {
        Ok(d) => d,
        Err(e) => {
            eprintln!(
                "Error reading dictionary '{}': {e}",
                args.dict_file.display()
            );
            process::exit(2);
        }
    };
    if args.word_count.get() > dict.len() {
        eprintln!(
            "Error: Requested word count is {} but dictionary contains only {} words",
            args.word_count,
            dict.len()
        );
        process::exit(3);
    }

    let mut rng = rand::rng();
    let separators: Vec<char> = args.separator.chars().collect();

    // generate passwords
    for _ in 0..args.pw_count.get() {
        // choose random separator between words
        let sep_words: char = *separators
            .choose(&mut rng)
            .expect("Separator must not be empty");

        // choose random separator for start/end
        let sep_edge: char = *separators
            .choose(&mut rng)
            .expect("Separator must not be empty");

        // chose random numbers for start/end
        let num_start = rng.random_range(1..=99);
        let num_end = rng.random_range(1..=99);

        // choose random words
        let words: Vec<&str> = dict
            .sample(&mut rng, args.word_count.get())
            .map(|s| s.as_str())
            .collect();

        // construct password
        let password = format!(
            "{num_start:02}{sep_edge}{}{sep_edge}{num_end:02}",
            words
                .iter()
                .enumerate()
                .map(|(i, &word)| {
                    if i % 2 == 1 {
                        word.to_uppercase()
                    } else {
                        word.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(&sep_words.to_string())
        );

        println!("{password}");
    }
}
