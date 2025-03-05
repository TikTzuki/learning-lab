use std::collections::HashMap;

#[derive(Debug)]
struct ValidAnagram {}


impl ValidAnagram {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut m: HashMap<char, usize> = HashMap::new();
        for k in s.chars() {
            *m.entry(k).or_insert(0) += 1;
        }

        let mut m2: HashMap<char, usize> = HashMap::new();
        for k in t.chars() {
            *m2.entry(k).or_insert(0) += 1;
        }

        return m == m2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let s = "car".parse().unwrap();
        let t = "rat".parse().unwrap();
        let result = ValidAnagram::is_anagram(s, t);
        println!("{:?}", result);
    }
}
