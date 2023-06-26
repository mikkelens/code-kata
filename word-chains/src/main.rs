use std::collections::HashSet;
use std::env;

const WORDLIST: &str = include_str!("wordlist.txt");

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (first, second) = match args.len() {
        2 => {
            let mut words = args.into_iter();
            (words.next().expect("First missing?"),
            words.next().expect("Second missing?"))
        },
        wrong_arg_count => {
            if wrong_arg_count < 2 {
                eprintln!("Not enough arguments specified. Amount required: 2");
            } else {
                eprintln!("Too many arguments specified [{}]. Amount required: 2", wrong_arg_count);
            }
            return;
        }
    };
    let all_words: HashSet<&str> = WORDLIST.lines().collect();
    if let Some(path) = shortest_word_chains_recursive(&vec![], &None, &first, &second, &all_words) {
        print!("Shortest found path from '{}' to '{}': ", first, second);
        display_path(&path);
    } else {
        eprintln!("No way to traverse from '{}' to '{}', according to this algorithm.", first, second);
    }
}
fn display_path(path: &WordPath) -> String {
    format!("[\n    {}\n]", path.join(",\n    "))
}

type WordPath = Vec<String>;
fn shortest_word_chains_recursive(chain_history_ref: &WordPath, shortest_path_ref: &Option<WordPath>, current_word: &str, target_word_ref: &str, dictionary_ref: &HashSet<&str>) -> Option<WordPath> {
    assert_eq!(current_word.len(), target_word_ref.len());
    let chain_with_word = {
        let mut new_chain_untill_now = chain_history_ref.clone();
        new_chain_untill_now.push(current_word.to_string());
        new_chain_untill_now
    };
    // println!("Chain untill this point: {}", chain_with_word.join(", "));
    if current_word == target_word_ref {
        return Some(chain_with_word); // word reached, a path can be returned
    }
    if chain_with_word.len() >= 10
        || shortest_path_ref.as_ref().is_some_and(|shortest| chain_with_word.len() >= shortest.len()) {
        return None;
    }

    let current_chars: Vec<char> = current_word.chars().collect();
    let target_chars: Vec<char> = target_word_ref.chars().collect();
    // try the straight forward ways towards target word
    for i in 0..target_word_ref.len() {
        let new_word = {
            let mut new_chars = current_chars.clone();
            new_chars[i] = target_chars[i];
            String::from_iter(new_chars.into_iter())
        };
        if chain_with_word.contains(&new_word) {
            continue;
        }
        println!("Trying new word: {}", new_word);
        if new_word == target_word_ref {
            // found target, going back up
            let mut succesful_chain = chain_with_word;
            succesful_chain.push(new_word);
            return Some(succesful_chain);
        }
        // continue
        if dictionary_ref.contains(new_word.as_str()) {
            if let Some(new_path) = shortest_word_chains_recursive(chain_history_ref, shortest_path_ref, &new_word, target_word_ref, dictionary_ref) {
                // found path at level, returning it back up
                return Some(new_path);
            }
        }
    }

    // try all other possible directions from current word
    for i in 0..current_word.len() {
        for c in 'a'..='z' {
            if c == target_chars[i] {
                continue; // direct follow, we have already tried this above
            }
            let new_word = {
                let mut new_chars = current_chars.clone();
                new_chars[i] = c;
                String::from_iter(new_chars.into_iter())
            };
            if chain_with_word.contains(&new_word) {
                continue; // we have already tried this direction or nothing changed
            }
            if dictionary_ref.contains(new_word.as_str()) {
                if let Some(new_path) = shortest_word_chains_recursive(&chain_with_word, shortest_path_ref, &new_word, target_word_ref, dictionary_ref) {
                    if let Some(existing_shortest) = &shortest_path_ref {
                        if new_path.len() < existing_shortest.len() {
                            return Some(new_path);
                        }
                    } else {
                        return Some(new_path);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAT_INTO_DOG_STR: &str = include_str!("sample_cat_into_dog.txt");
    #[test]
    fn can_turn_cat_into_dog_simple_way() {
        let all_words: HashSet<&str> = WORDLIST.lines().collect();
        let known_path: WordPath = CAT_INTO_DOG_STR.lines().map(|line| line.to_string()).collect();
        let cat = known_path.first().unwrap();
        let dog = known_path.last().unwrap();
        let path = shortest_word_chains_recursive(&vec![], &None, cat, dog, &all_words);
        assert!(path.is_some_and(|path| path.len() == known_path.len()));
    }
    #[test]
    fn can_turn_lead_into_gold_within_four_steps() {
        let all_words: HashSet<&str> = WORDLIST.lines().collect();
        let lead = "lead";
        let gold = "gold";
        let path = shortest_word_chains_recursive(&vec![], &None, lead, gold, &all_words);
        const SHORT_PATH_LEN: usize = 4;
        assert!(path.iter().any(|path| path.len() <= SHORT_PATH_LEN));
    }
    #[test]
    fn can_turn_ruby_into_code_within_six_steps() {
        let all_words: HashSet<&str> = WORDLIST.lines().collect();
        let lead = "ruby";
        let gold = "code";
        let all_paths = shortest_word_chains_recursive(&vec![], &None, lead, gold, &all_words);
        const SHORT_PATH_LEN: usize = 6; // note: 5 is also possible
        assert!(all_paths.iter().any(|path| path.len() <= SHORT_PATH_LEN));
    }
}