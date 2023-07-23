impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min3 = (None, None, None);

        for x in nums {
            match min3 {
                (_, None, _) => min3.1 = Some(x),
                (_, Some(b), None) if x < b => min3.1 = Some(x),
                (_, Some(b), None) if x > b => min3.2 = Some(x),
                (_, Some(b), Some(c)) if x > c => return true,
                (_, Some(b), Some(c)) if x > b => min3.2 = Some(x),
                (None, Some(b), Some(c)) if x < b => min3.0 = Some(x),
                (Some(a), Some(b), Some(c)) if x > a => min3 = (None, Some(a), Some(x)),
                (Some(a), Some(b), Some(c)) if x < a => min3.0 = Some(x),
                _ => (),
            }
        }

        false
    }
}
