use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let word_list = &args[1];
    let query_list = &args[2];
    println!("Using wordlist {} and querylist {}", word_list, query_list);

    let mut query = String::new();
    let mut words = String::new();
    let mut mappings = HashMap::new();

    let w = File::open(word_list)?;
    let mut buf_reader = BufReader::new(w);
    buf_reader.read_to_string(&mut words)?;

    let q = File::open(query_list)?;
    buf_reader = BufReader::new(q);
    buf_reader.read_to_string(&mut query)?;

    for word in words.lines() {
        let mut chars : Vec<char> = word.chars().collect();
        chars.sort();
        let sorted = chars.into_iter().collect::<String>();

        mappings.insert(sorted, word);
    }

    let mut result = String::new();
    let mut peek_query = query.lines().peekable();
    let mut m_word = peek_query.next();
    while m_word.is_some() {
        let word = m_word.unwrap();
        let mut chars : Vec<char> = word.chars().collect();
        chars.sort();
        let sorted = chars.into_iter().collect::<String>();

        let orig_word = mappings.get(&sorted).expect("Error word not found");
        result.push_str(orig_word);
        if peek_query.peek().is_some() {
            result.push_str(",");
        }
        m_word = peek_query.next();
    }

    println!("{}", result);

    Ok(())
}
