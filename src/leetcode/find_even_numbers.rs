use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..len {
            for j in 0..len {
                for k in 0..len {
                    if (i == k || k == j || i == j) {
                        continue;
                    } else if (digits[i] == 0) {
                        continue;
                    } else {
                        let v: i32 = digits[i] * 100 + digits[j] * 10 + digits[k] * 1;
                        if (v % 2 == 0) {
                            set.insert(v);
                        }
                    }
                }
            }
        }
        let mut nums: Vec<i32> = set.into_iter().collect();
        nums.sort();
        return nums;
    }
}
