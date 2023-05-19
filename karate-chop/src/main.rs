use std::cmp::Ordering;
use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn chop_flat_looping(target: usize, sorted_array: &[usize]) -> Option<usize> {
    println!("\n> Starting search (looping). Target: {target}, Array: {sorted_array:?}.");
    
    let mut search_range = 0..sorted_array.len();
    loop {
        let range_len = search_range.len();
        dbg!(range_len);
        if range_len == 0 {
            println!("< Reached root: Nothing left in array.");
            return None; 
        }
        
        let range_middle_offset = range_len / 2;
        let array_search_index = search_range.start + range_middle_offset;
        dbg!(array_search_index);
        
        let hit = sorted_array[array_search_index];
        dbg!(hit);
        match hit.cmp(&target) {
            Ordering::Equal => {
                println!("< Found target at index [{array_search_index}].");
                return Some(array_search_index);
            }
            _ if range_len == 1 => {
                println!("< Reached root: Last element in array did not match.");
                return None;
            },
            Ordering::Less => search_range.start = array_search_index + 1,
            Ordering::Greater => search_range.end = array_search_index,
        };
    }
}
#[allow(dead_code)]
fn chop_stack_recursive(target: usize, sorted_array: &[usize]) -> Option<usize> {
    println!("\n> Starting search (recursive). Target: {target}, Array: {sorted_array:?}.");
    
    fn chop_recursive(target: usize, sorted_array: &[usize], search_range: Range<usize>) -> Option<usize> {
        let range_len = search_range.len();
        dbg!(range_len);
        if range_len == 0 {
            println!("< Reached root: Nothing left in array.");
            return None;
        }
        
        let range_middle_offset = range_len / 2;
        let array_search_index = search_range.start + range_middle_offset;
        dbg!(array_search_index);
        
        let hit = sorted_array[array_search_index];
        dbg!(hit);
        match hit.cmp(&target) {
            Ordering::Equal => {
                println!("< Found target at index [{array_search_index}].");
                Some(array_search_index)
            },
            _ if range_len == 1 => {
                println!("< Reached root: Last element in array did not match.");
                None
            },
            Ordering::Less => chop_recursive(target, sorted_array, array_search_index..search_range.len()),
            Ordering::Greater => chop_recursive(target, sorted_array, 0..array_search_index),
        }
    }
    
    chop_recursive(target, sorted_array, 0..sorted_array.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flat_iterative() {
        test_implementation(chop_flat_looping);
    }
    #[test]
    fn test_stack_recursive() {
        test_implementation(chop_stack_recursive);
    }
    struct Query { target: usize, array: &'static [usize]}
    struct TestCase { expected_result_index: Option<usize>, search: Query }
    fn test_implementation(implementation: fn(usize, &[usize]) -> Option<usize>) {
        for case in TEST_CASES {
            assert_eq!(case.expected_result_index, implementation(case.search.target, case.search.array));
        }
    }
    const TEST_CASES: &[TestCase] = &[
        TestCase { expected_result_index: None, search: Query { target: 3, array: &[] } },
        TestCase { expected_result_index: None, search: Query { target: 3, array: &[1] } },
        TestCase { expected_result_index: Some(0), search: Query { target: 1, array: &[1] } },

        TestCase { expected_result_index: Some(0), search: Query { target: 1, array: &[1, 3, 5] } },
        TestCase { expected_result_index: Some(1), search: Query { target: 3, array: &[1, 3, 5] } },
        TestCase { expected_result_index: Some(2), search: Query { target: 5, array: &[1, 3, 5] } },
        TestCase { expected_result_index: None, search: Query { target: 0, array: &[1, 3, 5] } },
        TestCase { expected_result_index: None, search: Query { target: 2, array: &[1, 3, 5] } },
        TestCase { expected_result_index: None, search: Query { target: 4, array: &[1, 3, 5] } },
        TestCase { expected_result_index: None, search: Query { target: 6, array: &[1, 3, 5] } },

        TestCase { expected_result_index: Some(0), search: Query { target: 1, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: Some(1), search: Query { target: 3, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: Some(2), search: Query { target: 5, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: Some(3), search: Query { target: 7, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: None, search: Query { target: 0, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: None, search: Query { target: 2, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: None, search: Query { target: 4, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: None, search: Query { target: 6, array: &[1, 3, 5, 7] } },
        TestCase { expected_result_index: None, search: Query { target: 8, array: &[1, 3, 5, 7] } },
    ];
}