impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let len = groups.len();
        let mut dp = vec![1; len];
        let mut prev = vec![-1; len];
        let mut max_index = 0;
        for i in 1..len {
            for j in 0..i {
                if Self::check(&words[i], &words[j]) && dp[j] + 1 > dp[i] && groups[i] != groups[j] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j as i32;
                }
            }
            if dp[i] > dp[max_index] {
                max_index = i;
            }
        }
        let mut result = Vec::new();
        let mut i = max_index as i32;
        while i >= 0 {
            result.push(words[i as usize].clone());
            i = prev[i as usize];
        }
        result.reverse();
        result
    }

    fn check(s1: &String, s2: &String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut diff = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }
        diff == 1
    }
}
