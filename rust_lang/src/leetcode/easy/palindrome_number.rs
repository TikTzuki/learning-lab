#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s: String = x.to_string();
        if s.len() == 1 {
            return true;
        }
        if s.as_bytes()[0] == '-' as u8 {
            return false;
        }

        for i in 0..s.len() / 2 {
            if s.as_bytes()[i] != s.as_bytes()[s.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        n = { 3443, 141, -131, 4564 },
        expected = { true, true, false, false }
    )]
    fn it_works(n: i32, expected: bool) {
        let result = Solution::is_palindrome(n);
        assert_eq!(result, expected);
    }
}
