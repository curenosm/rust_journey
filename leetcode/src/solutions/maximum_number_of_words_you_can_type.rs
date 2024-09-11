use crate::Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let words: Vec<&str> = text.split(" ").into_iter().collect();
        let mut invalid = 0;

        'inner: for word in words.iter() {
            'outer: for c in broken_letters.chars() {
                if word.contains(c) {
                    invalid += 1;
                    continue 'inner;
                }
            }
        }

        words.len() as i32 - invalid
    }
}
