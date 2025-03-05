use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<u64, Vec<String>> = HashMap::new();
        for str in strs {
            let anagram = Anagram::new(str);
            m.entry(anagram.hash)
                .or_insert_with(Vec::new)
                .insert(0, anagram.value);
        }
        m.values().cloned().collect()
    }
}
struct Anagram {
    words: [i32; 26],
    value: String,
    hash: u64,
}
impl Anagram {
    fn new(s: String) -> Self {
        let word_array = s.chars().fold([0; 26], |mut acc, c| {
            acc[c as usize - 97] += 1;
            acc
        });
        let hash_value = Self::calculate_hash(&word_array);
        Anagram {
            words: word_array,
            value: s,
            hash: hash_value,
        }
    }

    fn add(&mut self, c: char) {
        self.words[c as usize - 97] += 1;
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_anagram() {
        let mut m: HashMap<u64, Vec<String>> = HashMap::new();

        let mut a = Anagram::new(String::from("ate"));

        let mut b = Anagram::new(String::from("eat"));

        m.entry(a.hash)
            .or_insert_with(Vec::new)
            .insert(0, a.value);
        m.entry(b.hash)
            .or_insert_with(Vec::new)
            .insert(0, b.value);

        println!("{}", a.hash == b.hash);
        println!("{:?}", m);
    }
    #[test]
    fn run() {

        let strs = vec![
            "eat".parse().unwrap(),
            "tea".parse().unwrap(),
            "tan".parse().unwrap(),
            "ate".parse().unwrap(),
            "nat".parse().unwrap(),
            "bat".parse().unwrap()];
        let result = Solution::group_anagrams(strs);
        println!("{:?}", result);
    }
}