// Given a paragraph and a list of banned words,
// return the most frequent word that is not in the list of banned words.
// It is guaranteed there is at least one word that isn't banned,
// and that the answer is unique.

// Words in the list of banned words are given in lowercase,
// and free of punctuation.
// Words in the paragraph are not case sensitive.
// The answer is in lowercase.

// Example:
// Input:
// paragraph = "Bob hit a ball, the hit BALL flew far after it was hit."
// banned = ["hit"]
// Output: "ball"
// Explanation:
// "hit" occurs 3 times, but it is a banned word.
// "ball" occurs twice (and no other word does),
// so it is the most frequent non-banned word in the paragraph.
// Note that words in the paragraph are not case sensitive,
// that punctuation is ignored (even if adjacent to words, such as "ball,"),
// and that "hit" isn't the answer even though it occurs more because it is banned.

// Note:
// 1 <= paragraph.length <= 1000.
// 0 <= banned.length <= 100.
// 1 <= banned[i].length <= 10.
// The answer is unique, and written in lowercase
// (even if its occurrences in paragraph may have uppercase symbols,
// and even if it is a proper noun.)
// paragraph only consists of letters, spaces, or the punctuation symbols !?',;.
// There are no hyphens or hyphenated words.
// Words only consist of letters, never apostrophes or other punctuation symbols.

use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let pa = paragraph
            .to_ascii_lowercase()
            .replace(['?', '!', ';', '.', ',', '\''].as_ref(), " ");
        let banned_set: HashSet<String> = banned.into_iter().collect();
        let mut words: HashMap<String, i32> = HashMap::new();
        for s in pa.split(' ').filter(|&s| !s.is_empty()) {
            if !banned_set.contains(s) {
                if let Some(x) = words.get_mut(s) {
                    *x += 1;
                } else {
                    words.insert(String::from(s), 1);
                }
            }
        }
        words
            .iter()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap()
            .0
            .to_string()
    }
}

fn main() {
    let paragraph = String::from("Bob hit a ball, the hit BALL flew far after it was hit.");
    let banned = vec![String::from("hit")];
    println!("paragragh: \"{}\"", paragraph.clone());
    println!("banned: {:?}", banned.clone());
    println!("-> \"{}\"", Solution::most_common_word(paragraph, banned));
}
