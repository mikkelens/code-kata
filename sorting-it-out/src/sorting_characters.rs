#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut vec: Vec<char> = "When not studying nuclear physics, Bambi likes to play beach volleyball."
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let expected_output: Vec<char> = "aaaaabbbbcccdeeeeeghhhiiiiklllllllmnnnnooopprsssstttuuvwyyyy"
            .chars()
            .collect();
        vec.sort_unstable();
        assert_eq!(vec, expected_output);
    }
}