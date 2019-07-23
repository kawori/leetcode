// Given words first and second,
// consider occurrences in some text of the form "first second third", where
// second comes immediately after first, and third comes immediately after second.

// For each such occurrence, add "third" to the answer, and return the answer.

// Example 1:
// Input: text = "alice is a good girl she is a good student", first = "a", second = "good"
// Output: ["girl","student"]

// Example 2:
// Input: text = "we will we will rock you", first = "we", second = "will"
// Output: ["we","rock"]

// Note:
// 1 <= text.length <= 1000
// text consists of space separated words, where each word consists of lowercase English letters.
// 1 <= first.length, second.length <= 10
// first and second consist of lowercase English letters.

struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let a = text.split(' ');
        let b = a.clone();
        let c = a.clone();
        let ret: Vec<String> = a
            .zip(b.skip(1))
            .zip(c.skip(2))
            .filter(|((x, y), _)| *x == first && *y == second)
            .map(|((_, _), z)| String::from(z))
            .collect();
        ret
    }
}

fn main() {
    let text = String::from("alice is a good girl she is a good student");
    let first = String::from("a");
    let second = String::from("good");
    println!("text: \"alice is a good girl she is a good student\"");
    println!("first: \"a\", second: \"good\"");
    println!("-> {:?}", Solution::find_ocurrences(text, first, second));

    let text = String::from("we will we will rock you");
    let first = String::from("we");
    let second = String::from("will");
    println!("\ntext: \"we will we will rock you\"");
    println!("first: \"we\", second: \"will\"");
    println!("-> {:?}", Solution::find_ocurrences(text, first, second));
}
