impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let len = words.len();
        for i in 0..len {
            if (i == 0 || (groups[i] != groups[i - 1])) {
                res.push(words[i].clone());
            }
        }
        res
    }
}
