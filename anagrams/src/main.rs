
fn main() {
    let input = include_str!("wordlist.txt").to_ascii_lowercase();
    let word_list: Vec<&str> = input.lines().collect();

    for word in &word_list {
        if word.contains("'s") {
            continue;
        }

        let mut anagrams: Vec<String> = Vec::new();

        let word_chars: Vec<char> = word.chars().collect();
        let possible_word_matches: Vec<&str> = word_list.iter().filter(|word_in_list| word_in_list.len() == word.len() ).copied().collect();
        for valid_combination in char_combinations(Vec::new(), word_chars, possible_word_matches) {
            dbg!(&valid_combination);
            anagrams.push(valid_combination);
        }
        println!("{} has {} anagrams: {:?}", word, anagrams.len(), anagrams);
    }
}

fn char_combinations(mut selected_chars: Vec<char>, remaining_chars: Vec<char>, matching_valid_words: Vec<&str>) -> Vec<String> {
    if remaining_chars.len() == 1 {
        selected_chars.push(*remaining_chars.first().unwrap());
        let final_word: String = selected_chars.into_iter().collect();
        if matching_valid_words.contains(&final_word.as_str()) {
            return vec![final_word];
        }
        return vec![];
    }
    
    let mut words: Vec<String> = Vec::new();
    for selected_char in &remaining_chars {
        let mut selected_chars_for_selection = selected_chars.clone();
        selected_chars_for_selection.push(*selected_char);
        
        let word_so_far: String = selected_chars_for_selection.iter().collect();
        let remaining_matching_words: Vec<&str> = matching_valid_words.iter().filter(|valid_word| valid_word.contains(&word_so_far)).copied().collect();
        if remaining_chars.is_empty() {
            continue;
        }
        
        let mut remaining_chars_for_selection = remaining_chars.clone();
        remaining_chars_for_selection.retain(|c| c != selected_char);
        
        let mut new_words = char_combinations(selected_chars_for_selection, remaining_chars_for_selection, remaining_matching_words);
        words.append(&mut new_words);
    }

    words
}