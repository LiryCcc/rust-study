impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let len = arr.len();
        if (len < 3) {
            return false;
        }
        for mut i in 0..len - 2 {
            if arr[i] % 2 == 1 {
                if i <= len - 2 {
                    if arr[i + 1] % 2 == 1 && arr[i + 2] % 2 == 1 {
                        return true;
                    }
                }
            } else {
                i += 1;
            }
        }
        return false;
    }
}
