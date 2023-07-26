use std::collections::HashMap;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();

    let input = include_str!("wordlist.txt");
    let word_list: Vec<&str> = input.lines().collect();

    // create hashmap with keys as anagram connections (letters), values as all words with those letters
    let word_map = {
        let mut word_map: HashMap<String, Vec<&str>> = HashMap::new();
        for word in word_list {
            // dbg!(&word);
            let word_sorted = {
                let mut word_chars: Vec<char> = word.chars().collect();
                word_chars.sort_unstable();
                String::from_iter(word_chars)
            };
            // dbg!(&word_sorted);
            if word_map.contains_key(&word_sorted) {
                if !word_map.get(&word_sorted).unwrap().contains(&word) {
                    word_map.get_mut(&word_sorted).unwrap().push(word);
                    // println!("Added.");
                }
            } else {
                word_map.insert(word_sorted.clone(), vec![word]);
                // println!("Created.");
            }
        }
        word_map
    };
    println!("Stored all the words as a hashmap.");
    
    let anagram_map: HashMap<String, Vec<&str>> = word_map.into_iter().filter(|(_, words)| words.len() > 1).collect();
    println!("Created anagram hashmap from word hashmap.");
    
    println!("Anagram hashmap contains {} keys, with {} values in total.", anagram_map.keys().len(), anagram_map.values().map(std::vec::Vec::len).sum::<usize>());
    
    let all_anagrams: Vec<(&String, &Vec<&str>)> = anagram_map.iter().collect();

    let largest_anagram_words = {
        let mut by_word_len = all_anagrams.clone();
        by_word_len.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        *by_word_len.first().unwrap()
    };
    println!("Largest anagram by word length ({} characters): {}", largest_anagram_words.0.len(), largest_anagram_words.1.join(", "));
    
    let largest_anagram_amount = {
        let mut by_word_len = all_anagrams.clone();
        by_word_len.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        *by_word_len.first().unwrap()
    };
    println!("Largest anagram by word amount ({} entries): {}", largest_anagram_amount.1.len(), largest_anagram_amount.1.join(", "));

    println!("\nProgram took {} ms to finish.", SystemTime::duration_since(&SystemTime::now(), start_time).unwrap().as_millis());
}