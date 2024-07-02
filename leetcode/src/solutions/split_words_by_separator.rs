use crate::Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .into_iter()
            .flat_map(|w| {
                w.split(separator)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
