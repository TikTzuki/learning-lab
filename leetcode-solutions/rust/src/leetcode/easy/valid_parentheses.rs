use std::collections::{HashMap, VecDeque};

struct ValidParentheses {}

impl ValidParentheses {
    pub fn is_valid(s: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert('(', ')');
        map.insert('[', ']');
        map.insert('{', '}');

        let mut char_stack: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            if !char_stack.is_empty() && *map.get(char_stack.back().unwrap()).unwrap() == c {
                char_stack.pop_back();
            } else {
                char_stack.push_back(c);
            }
        }
        return char_stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let result = ValidParentheses::is_valid(String::from("{}"));
        println!("{:?}", result);
    }
}
