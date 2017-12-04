use std::collections::HashSet;
use std::iter::FromIterator;

pub fn valid_count(phrases: Vec<Vec<String>>) -> usize {
    let mut phrases = phrases.clone();
    phrases.retain(|p| valid_passphrase(&p));
    phrases.len()
}

fn sort_word(word: &String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let s = String::from_iter(chars);
    s
}

fn valid_passphrase(phrase: &Vec<String>) -> bool {
    let mut words = HashSet::new();
    for word in phrase {
        let sorted_word = sort_word(&word);
        if words.contains(&sorted_word) {
            return false
        } else {
            words.insert(sorted_word);
        }
    }
    true
}
