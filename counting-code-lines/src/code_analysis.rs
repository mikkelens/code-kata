/// COMMENT RULES:
/// '//' means a comment untill line end
/// '/*' means a multiline comment started
/// '*/' means a multiline comment ended

const WHITE_SPACE: char = ' ';
const COMMENT_OUTER: char = '/';
const COMMENT_MULTI_INNER: char = '*';
const STR_LITERAL_EDGES: char = '\"';
const CHAR_EDGES: char = '\'';
#[derive(PartialEq, Debug)]
enum ReadState {
    OnWhiteSpace, // effective reset, base value
    OnSlash, // either symbol or comment of some kind, always counted as symbol if not connected to anything
    OnIdentifiedSymbol, // continue search in case of string literal
    InEndOfLineComment, // basically a card to skip to next line
    InMultilineComment { ending_comment: bool }, // potentially "least valuable", can continue on multiple lines
    InQuotation(char), // "most valuable", can potentially continue on multiple lines
}
impl ReadState {
    fn after_next_char_in_line(&self, next: char) -> ReadState {
        match *self {
            Self::OnWhiteSpace => match next {
                WHITE_SPACE => Self::OnWhiteSpace, // '  '
                COMMENT_OUTER => Self::OnSlash, // ' /'
                STR_LITERAL_EDGES | CHAR_EDGES => Self::InQuotation(next), // ' "'
                _ => Self::OnIdentifiedSymbol // ' X', could also be ' *'
            },
            Self::OnSlash => match next {
                WHITE_SPACE => Self::OnWhiteSpace, // '/_', might as well be identified symbol technically?
                COMMENT_OUTER => Self::InEndOfLineComment, // '//'
                COMMENT_MULTI_INNER => Self::InMultilineComment { ending_comment: false }, // '/*'                
                STR_LITERAL_EDGES | CHAR_EDGES => Self::InQuotation(next), // '/"'
                _ => Self::OnIdentifiedSymbol // '/X'
            },
            Self::OnIdentifiedSymbol => match next {
                COMMENT_OUTER => Self::OnSlash, // 'X/'
                STR_LITERAL_EDGES | CHAR_EDGES => Self::InQuotation(next), // 'X"'
                _ => Self::OnIdentifiedSymbol // ' X', could also be '_
            },
            Self::InEndOfLineComment => unreachable!(), // '/!', should never stay in this state
            Self::InMultilineComment { ending_comment } => match next {
                COMMENT_OUTER if ending_comment => Self::OnWhiteSpace, // '*/', use whitespace as reset
                c => Self::InMultilineComment { ending_comment: c == COMMENT_MULTI_INNER } // ' *' (true) or ' @' (false)
            },
            Self::InQuotation(boundary) => {
                if next == boundary { // characters do not matter if not closing quote '"'
                    Self::OnIdentifiedSymbol // '@"', still counts as symbol
                } else {
                    Self::InQuotation(boundary) // '@@', keep boundary in mind
                }
            },
        }
    }
    fn is_multi_line(&self) -> bool {
        matches!(self, ReadState::InMultilineComment { ending_comment: _ } | ReadState::InQuotation(_))
    }
}

#[must_use] pub fn count_valid_code_lines(literal: &str) -> u32 {
    // println!();

    let mut lines_with_symbol = 0;
    let mut prev_state = ReadState::OnWhiteSpace;

    for line_with_all_whitespace in literal.trim().lines() {
        // println!("\nBeginning line with state {:?}:\n{:?}", prev_state, line_with_all_whitespace);

        if !prev_state.is_multi_line() {
            prev_state = ReadState::OnWhiteSpace;
        }

        let mut symbol_in_line = false;

        let chars_with_min_whitespace = line_with_all_whitespace.trim().chars().collect::<Vec<char>>();
        for char_in_line in chars_with_min_whitespace {
            let new_state = prev_state.after_next_char_in_line(char_in_line);
        
            if new_state == ReadState::InEndOfLineComment { // reset in beginning of next loop
                prev_state = new_state;
                break;
            }
            
            let new_is_counted = matches!(new_state, ReadState::OnIdentifiedSymbol | ReadState::InQuotation(_));
            let prev_is_counted = prev_state == ReadState::OnSlash && new_state == ReadState::OnWhiteSpace;
            if new_is_counted || prev_is_counted {
                symbol_in_line = true;
            }
            prev_state = new_state;
            // loop continues because there could be state changes that affect the reading of next line
        }

        let prev_be_counted_anyways = prev_state == ReadState::OnSlash;
        if prev_be_counted_anyways {
            symbol_in_line = true;
        }

        if symbol_in_line {
            lines_with_symbol += 1;
            // println!("Line had symbol. Total lines with symbols so far: {lines_with_symbol}");
        } else {
            // println!("Line had no symbol.");
        }
    }

    lines_with_symbol
}

#[cfg(test)]
mod tests {
    use crate::code_analysis::*;

    const EXAMPLE_1: &str = include_str!("test_examples\\Dave.java");
    const EXAMPLE_1_OUT: u32 = 3;
    #[test]
    fn example_1() {
        assert_eq!(count_valid_code_lines(EXAMPLE_1), EXAMPLE_1_OUT);
    }
    
    const EXAMPLE_2: &str = include_str!("test_examples\\Hello.java");
    const EXAMPLE_2_OUT: u32 = 5;
    #[test]
    fn example_2() {
        assert_eq!(count_valid_code_lines(EXAMPLE_2), EXAMPLE_2_OUT);
    }
}