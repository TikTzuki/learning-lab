use std::collections::HashMap;

#[derive(Debug)]
struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        for (i, value) in nums.iter().enumerate() {
            let find_value = target - value;
            if hash.contains_key(&find_value) {
                return vec![hash[&find_value], (i as i32)];
            }
            hash.insert(value.clone(), i as i32);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::two_sum(vec![1, 2, 3, 4], 7);
        println!("{:?}", result);
    }
}
